use super::stack::Stack;
use crate::bytecode::{instruction::Instruction, ByteCode};
use crate::config::{OpCodes, Value, Visitor};

#[derive(Debug, Clone)]
struct InterpreterState {
    stack: Stack<Value>,
    vars: Vec<Value>,
    ip: usize,
}

impl InterpreterState {
    pub fn new(data_size: usize) -> Self {
        let mut ret = InterpreterState {
            stack: Stack::new(),
            vars: vec![],
            ip: 0,
        };
        ret.vars.resize(data_size, 0 as Value);
        ret
    }
    pub fn get_ip(&self) -> usize {
        self.ip
    }
    pub fn set_ip(&mut self, ip: Value) {
        self.ip = ip as usize;
    }
    pub fn next(&mut self) {
        self.ip += 1;
    }
    pub fn pop_value(&mut self) -> Result<Value, String> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(format!(
                "Runtime error: unable to process current instruction, ip = 0x{:02x}: no value on stack",
                self.ip
            )),
        }
    }
    pub fn push_value(&mut self, v: Value) {
        self.stack.push(v);
    }
    pub fn add_var(&mut self, address: Value, value: Value) -> Result<(), String> {
        if address as usize >= self.vars.len() {
            return Err(format!(
                "Runtime error: unable to process current instruction, ip = 0x{:02x}: invalid variable address 0x{:02x}",
                self.ip,
                address
            ));
        }
        self.vars[address as usize] = value;
        return Ok(());
    }
    pub fn read_var(&mut self, address: Value) -> Result<Value, String> {
        if address as usize >= self.vars.len() {
            return Err(format!(
                "Runtime error: unable to process current instruction, ip = 0x{:02x}: invalid variable address 0x{:02x}",
                self.ip,
                address
            ));
        }
        Ok(self.vars[address as usize])
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    bytecode: ByteCode,
}

impl Interpreter {
    pub fn new(bytecode: ByteCode) -> Self {
        Interpreter { bytecode }
    }
    pub fn interpret(&mut self) -> Result<Option<Value>, String> {
        let mut ctx = InterpreterState::new(self.bytecode.get_data_size());
        loop {
            let mut instruction = match self.bytecode.get_instruction(ctx.get_ip()) {
                Some(instruction) => instruction.clone(),
                None => return Ok(None),
            };
            let opcode = match instruction.opcode {
                Some(ref opcode) => opcode.clone(),
                None => return Err("Invalid instruction: empty opcode".to_string()),
            };

            match opcode {
                OpCodes::LOAD => instruction.visit_load(&mut ctx),
                OpCodes::WRT => instruction.visit_wrt(&mut ctx),
                OpCodes::READ => instruction.visit_read(&mut ctx),
                OpCodes::ADD => instruction.visit_add(&mut ctx),
                OpCodes::MULT => instruction.visit_mult(&mut ctx),
                OpCodes::TEGT => instruction.visit_test_gt(&mut ctx),
                OpCodes::TELT => instruction.visit_test_lt(&mut ctx),
                OpCodes::TEEQ => instruction.visit_test_eq(&mut ctx),
                OpCodes::GOTO => instruction.visit_goto(&mut ctx),
                OpCodes::DUP => instruction.visit_dup(&mut ctx),
                OpCodes::POP => instruction.visit_pop(&mut ctx),
                OpCodes::RTN => {
                    instruction.visit_rtn(&mut ctx)?;
                    return ctx.pop_value().map(Some);
                }
            }?;
        }
    }
}

impl Visitor<InterpreterState> for Instruction {
    fn visit_load(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args == None {
            return Err("Invalid LOAD instruction: empty args".to_string());
        }
        if self.args.as_ref().unwrap().len() != 1 {
            return Err(format!(
                "Invalid LOAD instruction: expected 1 argument, got {}",
                self.args.as_ref().unwrap().len()
            ));
        }
        ctx.push_value(self.args.as_ref().unwrap()[0].clone());
        ctx.next();
        Ok(())
    }
    fn visit_wrt(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args == None {
            return Err("Invalid WRT instruction: empty args".to_string());
        }
        if self.args.as_ref().unwrap().len() != 1 {
            return Err(format!(
                "Invalid WRT instruction: expected 1 argument, got {}",
                self.args.as_ref().unwrap().len()
            ));
        }
        let value = ctx.pop_value()?;
        ctx.add_var(self.args.as_ref().unwrap()[0].clone(), value)?;
        ctx.next();
        Ok(())
    }
    fn visit_read(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args == None {
            return Err("Invalid READ instruction: empty args".to_string());
        }
        if self.args.as_ref().unwrap().len() != 1 {
            return Err(format!(
                "Invalid READ instruction: expected 1 argument, got {}",
                self.args.as_ref().unwrap().len()
            ));
        }
        let value = ctx.read_var(self.args.as_ref().unwrap()[0].clone())?;
        ctx.push_value(value);
        ctx.next();
        Ok(())
    }
    fn visit_add(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args != None {
            return Err("Invalid ADD instruction: unexpected args".to_string());
        }
        let v1 = ctx.pop_value()?;
        let v2 = ctx.pop_value()?;
        ctx.push_value(v1 + v2);
        ctx.next();
        Ok(())
    }
    fn visit_mult(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args != None {
            return Err("Invalid MULT instruction: unexpected args".to_string());
        }
        let v1 = ctx.pop_value()?;
        let v2 = ctx.pop_value()?;
        ctx.push_value(v1 * v2);
        ctx.next();
        Ok(())
    }
    fn visit_rtn(&mut self, _ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args != None {
            return Err("Invalid RTN instruction: unexpected args".to_string());
        }
        Ok(())
    }
    fn visit_test_eq(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args != None {
            return Err("Invalid TEEQ instruction: unexpected args".to_string());
        }
        let v1 = ctx.pop_value()?;
        let v2 = ctx.pop_value()?;
        ctx.push_value((v1 == v2) as Value);
        ctx.next();
        Ok(())
    }
    fn visit_test_gt(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args != None {
            return Err("Invalid TEGT instruction: unexpected args".to_string());
        }
        let v1 = ctx.pop_value()?;
        let v2 = ctx.pop_value()?;
        ctx.push_value((v1 > v2) as Value);
        ctx.next();
        Ok(())
    }
    fn visit_test_lt(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args != None {
            return Err("Invalid TELT instruction: unexpected args".to_string());
        }
        let v1 = ctx.pop_value()?;
        let v2 = ctx.pop_value()?;
        ctx.push_value((v1 < v2) as Value);
        ctx.next();
        Ok(())
    }
    fn visit_goto(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args == None {
            return Err("Invalid GOTO instruction: empty args".to_string());
        }
        if self.args.as_ref().unwrap().len() != 1 {
            return Err(format!(
                "Invalid GOTO instruction: expected 1 argument, got {}",
                self.args.as_ref().unwrap().len()
            ));
        }
        let v = ctx.pop_value()?;
        if v == 0 {
            ctx.next();
        } else {
            ctx.set_ip(self.args.as_ref().unwrap()[0].clone());
        }
        Ok(())
    }
    fn visit_dup(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args != None {
            return Err("Invalid DUP instruction: unexpected args".to_string());
        }
        let v = ctx.pop_value()?;
        ctx.push_value(v);
        ctx.push_value(v);
        ctx.next();
        Ok(())
    }
    fn visit_pop(&mut self, ctx: &mut InterpreterState) -> Result<(), String> {
        if self.args != None {
            return Err("Invalid POP instruction: unexpected args".to_string());
        }
        ctx.pop_value()?;
        ctx.next();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::instruction::Instruction;
    use crate::config::OpCodes;

    #[test]
    fn test_interpret_load() {
        let value_to_load: Value = 0x1;
        let mut ctx = InterpreterState::new(0);
        let mut load_instr = Instruction {
            opcode: Some(OpCodes::LOAD),
            args: Some(vec![value_to_load]),
        };

        let result = load_instr.visit_load(&mut ctx);

        assert!(result.is_ok());
        let on_stack = ctx.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), value_to_load);
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_load_empty_args() {
        let mut ctx = InterpreterState::new(0);
        let mut load_instr = Instruction {
            opcode: Some(OpCodes::LOAD),
            args: None,
        };

        let result = load_instr.visit_load(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid LOAD instruction: empty args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_load_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut load_instr = Instruction {
            opcode: Some(OpCodes::LOAD),
            args: Some(vec![0x1, 0x1]),
        };

        let result = load_instr.visit_load(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid LOAD instruction: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn test_interpret_wrt() {
        let value_to_load: Value = 0x2;
        let mut ctx = InterpreterState::new(1);
        ctx.push_value(value_to_load);

        let mut wrt_instr = Instruction {
            opcode: Some(OpCodes::WRT),
            args: Some(vec![0x0]),
        };

        let result = wrt_instr.visit_wrt(&mut ctx);
        assert!(result.is_ok());
        assert_eq!(ctx.vars.len(), 1);
        assert_eq!(ctx.vars[0], value_to_load);
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_wrt_empty_args() {
        let mut ctx = InterpreterState::new(0);
        let mut wrt_instr = Instruction {
            opcode: Some(OpCodes::WRT),
            args: None,
        };

        let result = wrt_instr.visit_wrt(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid WRT instruction: empty args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_wrt_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut wrt_instr = Instruction {
            opcode: Some(OpCodes::WRT),
            args: Some(vec![0x1, 0x1]),
        };

        let result = wrt_instr.visit_wrt(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid WRT instruction: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_wrt_empty_stack() {
        let mut ctx = InterpreterState::new(0);
        let mut wrt_instr = Instruction {
            opcode: Some(OpCodes::WRT),
            args: Some(vec![0x2]),
        };

        let result = wrt_instr.visit_wrt(&mut ctx);
        assert_eq!(
            result,
            Err("Runtime error: unable to process current instruction, ip = 0x00: no value on stack".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_wrt_empty_data() {
        let value_to_load: Value = 0x2;
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(value_to_load);

        let mut wrt_instr = Instruction {
            opcode: Some(OpCodes::WRT),
            args: Some(vec![0x0]),
        };

        let result = wrt_instr.visit_wrt(&mut ctx);
        assert_eq!(
            result,
            Err(
                "Runtime error: unable to process current instruction, ip = 0x00: invalid variable address 0x00"
                    .to_string(),
            )
        );
    }

    #[test]
    fn test_interpret_read() {
        let value_to_load: Value = 0x2;
        let mut ctx = InterpreterState::new(1);
        ctx.vars[0] = value_to_load;

        let mut read_instr = Instruction {
            opcode: Some(OpCodes::READ),
            args: Some(vec![0x0]),
        };

        let result = read_instr.visit_read(&mut ctx);
        assert!(result.is_ok());
        let on_stack = ctx.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), value_to_load);
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_read_empty_args() {
        let mut ctx = InterpreterState::new(0);
        let mut read_instr = Instruction {
            opcode: Some(OpCodes::READ),
            args: None,
        };

        let result = read_instr.visit_read(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid READ instruction: empty args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_read_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut read_instr = Instruction {
            opcode: Some(OpCodes::READ),
            args: Some(vec![0x1, 0x1]),
        };

        let result = read_instr.visit_read(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid READ instruction: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_read_empty_data() {
        let mut ctx = InterpreterState::new(0);

        let mut read_instr = Instruction {
            opcode: Some(OpCodes::READ),
            args: Some(vec![0x0]),
        };

        let result = read_instr.visit_read(&mut ctx);
        assert_eq!(
            result,
            Err(
                "Runtime error: unable to process current instruction, ip = 0x00: invalid variable address 0x00"
                    .to_string(),
            )
        );
    }

    #[test]
    fn test_interpret_add() {
        let v1: Value = 0x1;
        let v2: Value = 0x2;
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(v1);
        ctx.push_value(v2);

        let mut add_instr = Instruction {
            opcode: Some(OpCodes::ADD),
            args: None,
        };

        let result = add_instr.visit_add(&mut ctx);
        assert!(result.is_ok());
        let on_stack = ctx.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), v1 + v2);
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_add_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut add_instr = Instruction {
            opcode: Some(OpCodes::ADD),
            args: Some(vec![0x1, 0x1]),
        };

        let result = add_instr.visit_add(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid ADD instruction: unexpected args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_add_empty_stack() {
        let mut ctx = InterpreterState::new(0);
        let mut add_instr = Instruction {
            opcode: Some(OpCodes::ADD),
            args: None,
        };

        let result = add_instr.visit_add(&mut ctx);
        assert_eq!(
            result,
            Err("Runtime error: unable to process current instruction, ip = 0x00: no value on stack".to_string())
        );
    }

    #[test]
    fn test_interpret_mult() {
        let v1: Value = 0x1;
        let v2: Value = 0x2;
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(v1);
        ctx.push_value(v2);

        let mut mult_instr = Instruction {
            opcode: Some(OpCodes::MULT),
            args: None,
        };

        let result = mult_instr.visit_mult(&mut ctx);
        assert!(result.is_ok());
        let on_stack = ctx.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), v1 * v2);
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_mult_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut mult_instr = Instruction {
            opcode: Some(OpCodes::MULT),
            args: Some(vec![0x1, 0x1]),
        };

        let result = mult_instr.visit_mult(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid MULT instruction: unexpected args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_mult_empty_stack() {
        let mut ctx = InterpreterState::new(0);
        let mut mult_instr = Instruction {
            opcode: Some(OpCodes::MULT),
            args: None,
        };

        let result = mult_instr.visit_mult(&mut ctx);
        assert_eq!(
            result,
            Err("Runtime error: unable to process current instruction, ip = 0x00: no value on stack".to_string())
        );
    }

    #[test]
    fn test_interpret_rtn() {
        let mut ctx = InterpreterState::new(0);

        let mut rtn_instr = Instruction {
            opcode: Some(OpCodes::RTN),
            args: None,
        };

        let result = rtn_instr.visit_rtn(&mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interpret_bad_rtn_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut rtn_instr = Instruction {
            opcode: Some(OpCodes::RTN),
            args: Some(vec![0x1]),
        };

        let result = rtn_instr.visit_rtn(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid RTN instruction: unexpected args".to_string())
        );
    }

    #[test]
    fn test_interpret_tegt() {
        let v1: Value = 0x1;
        let v2: Value = 0x2;
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(v1);
        ctx.push_value(v2);

        let mut tegt_instr = Instruction {
            opcode: Some(OpCodes::TEGT),
            args: None,
        };

        let result = tegt_instr.visit_test_gt(&mut ctx);
        assert!(result.is_ok());
        let on_stack = ctx.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), 0x1);
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_tegt_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut tegt_instr = Instruction {
            opcode: Some(OpCodes::TEGT),
            args: Some(vec![0x1, 0x1]),
        };

        let result = tegt_instr.visit_test_gt(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid TEGT instruction: unexpected args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_tegt_empty_stack() {
        let mut ctx = InterpreterState::new(0);
        let mut tegt_instr = Instruction {
            opcode: Some(OpCodes::TEGT),
            args: None,
        };

        let result = tegt_instr.visit_test_gt(&mut ctx);
        assert_eq!(
            result,
            Err("Runtime error: unable to process current instruction, ip = 0x00: no value on stack".to_string())
        );
    }

    #[test]
    fn test_interpret_telt() {
        let v1: Value = 0x1;
        let v2: Value = 0x2;
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(v1);
        ctx.push_value(v2);

        let mut telt_instr = Instruction {
            opcode: Some(OpCodes::TELT),
            args: None,
        };

        let result = telt_instr.visit_test_lt(&mut ctx);
        assert!(result.is_ok());
        let on_stack = ctx.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), 0x0);
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_telt_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut telt_instr = Instruction {
            opcode: Some(OpCodes::TELT),
            args: Some(vec![0x1, 0x1]),
        };

        let result = telt_instr.visit_test_lt(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid TELT instruction: unexpected args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_telt_empty_stack() {
        let mut ctx = InterpreterState::new(0);
        let mut telt_instr = Instruction {
            opcode: Some(OpCodes::TELT),
            args: None,
        };

        let result = telt_instr.visit_test_lt(&mut ctx);
        assert_eq!(
            result,
            Err("Runtime error: unable to process current instruction, ip = 0x00: no value on stack".to_string())
        );
    }

    #[test]
    fn test_interpret_teeq() {
        let v1: Value = 0x1;
        let v2: Value = 0x1;
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(v1);
        ctx.push_value(v2);

        let mut teeq_instr = Instruction {
            opcode: Some(OpCodes::TEEQ),
            args: None,
        };

        let result = teeq_instr.visit_test_eq(&mut ctx);
        assert!(result.is_ok());
        let on_stack = ctx.pop_value();
        assert!(on_stack.is_ok());
        assert_eq!(on_stack.ok().unwrap(), 0x1);
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_teeq_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut teeq_instr = Instruction {
            opcode: Some(OpCodes::TEEQ),
            args: Some(vec![0x1, 0x1]),
        };

        let result = teeq_instr.visit_test_eq(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid TEEQ instruction: unexpected args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_teeq_empty_stack() {
        let mut ctx = InterpreterState::new(0);
        let mut teeq_instr = Instruction {
            opcode: Some(OpCodes::TEEQ),
            args: None,
        };

        let result = teeq_instr.visit_test_eq(&mut ctx);
        assert_eq!(
            result,
            Err("Runtime error: unable to process current instruction, ip = 0x00: no value on stack".to_string())
        );
    }

    #[test]
    fn test_interpret_goto_false() {
        let desired_ip = 0x5;
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(0x0);

        let mut goto_instr = Instruction {
            opcode: Some(OpCodes::GOTO),
            args: Some(vec![desired_ip]),
        };

        let result = goto_instr.visit_goto(&mut ctx);
        assert!(result.is_ok());
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_goto_true() {
        let desired_ip = 0x5;
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(0x1);

        let mut goto_instr = Instruction {
            opcode: Some(OpCodes::GOTO),
            args: Some(vec![desired_ip]),
        };

        let result = goto_instr.visit_goto(&mut ctx);
        assert!(result.is_ok());
        assert_eq!(ctx.ip, desired_ip as usize);
    }

    #[test]
    fn test_interpret_bad_goto_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut goto_instr = Instruction {
            opcode: Some(OpCodes::GOTO),
            args: Some(vec![0x1, 0x1]),
        };

        let result = goto_instr.visit_goto(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid GOTO instruction: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_goto_empty_args() {
        let mut ctx = InterpreterState::new(0);
        let mut goto_instr = Instruction {
            opcode: Some(OpCodes::GOTO),
            args: None,
        };

        let result = goto_instr.visit_goto(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid GOTO instruction: empty args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_goto_empty_stack() {
        let mut ctx = InterpreterState::new(0);
        let mut goto_instr = Instruction {
            opcode: Some(OpCodes::GOTO),
            args: Some(vec![0x1]),
        };

        let result = goto_instr.visit_goto(&mut ctx);
        assert_eq!(
            result,
            Err("Runtime error: unable to process current instruction, ip = 0x00: no value on stack".to_string())
        );
    }

    #[test]
    fn test_interpret_dup() {
        let v1: Value = 0x1;
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(v1);

        let mut dup_instr = Instruction {
            opcode: Some(OpCodes::DUP),
            args: None,
        };

        let result = dup_instr.visit_dup(&mut ctx);
        assert!(result.is_ok());
        let on_stack_1 = ctx.pop_value();
        let on_stack_2 = ctx.pop_value();
        assert!(on_stack_1.is_ok());
        assert!(on_stack_2.is_ok());
        assert_eq!(on_stack_1.ok().unwrap(), 0x1);
        assert_eq!(on_stack_2.ok().unwrap(), 0x1);
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_dup_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut dup_instr = Instruction {
            opcode: Some(OpCodes::DUP),
            args: Some(vec![0x1, 0x1]),
        };

        let result = dup_instr.visit_dup(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid DUP instruction: unexpected args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_dup_empty_stack() {
        let mut ctx = InterpreterState::new(0);
        let mut dup_instr = Instruction {
            opcode: Some(OpCodes::DUP),
            args: None,
        };

        let result = dup_instr.visit_dup(&mut ctx);
        assert_eq!(
            result,
            Err("Runtime error: unable to process current instruction, ip = 0x00: no value on stack".to_string())
        );
    }

    #[test]
    fn test_interpret_pop() {
        let mut ctx = InterpreterState::new(0);
        ctx.push_value(0x2);

        let mut pop_instr = Instruction {
            opcode: Some(OpCodes::POP),
            args: None,
        };

        let result = pop_instr.visit_pop(&mut ctx);
        assert!(result.is_ok());
        let on_stack = ctx.pop_value();
        assert_eq!(on_stack, Err("Runtime error: unable to process current instruction, ip = 0x01: no value on stack".to_string()));
        assert_eq!(ctx.ip, 0x1);
    }

    #[test]
    fn test_interpret_bad_pop_excessive_args() {
        let mut ctx = InterpreterState::new(0);
        let mut pop_instr = Instruction {
            opcode: Some(OpCodes::POP),
            args: Some(vec![0x1, 0x1]),
        };

        let result = pop_instr.visit_pop(&mut ctx);
        assert_eq!(
            result,
            Err("Invalid POP instruction: unexpected args".to_string())
        );
    }

    #[test]
    fn test_interpret_bad_pop_empty_stack() {
        let mut ctx = InterpreterState::new(0);
        let mut pop_instr = Instruction {
            opcode: Some(OpCodes::POP),
            args: None,
        };

        let result = pop_instr.visit_pop(&mut ctx);
        assert_eq!(
            result,
            Err("Runtime error: unable to process current instruction, ip = 0x00: no value on stack".to_string())
        );
    }
}
