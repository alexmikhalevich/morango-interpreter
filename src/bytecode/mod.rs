mod context;
pub mod instruction;

use crate::config::{OpCodes, Value, Visitor};
use context::Context;
use instruction::Instruction;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteCode {
    instructions: Vec<Instruction>,
    data_size: usize,
}

impl ByteCode {
    pub fn transpile(source_file: &str) -> Result<Self, String> {
        let file = match File::open(source_file) {
            Ok(file) => file,
            Err(e) => return Err(format!("Unable to open file: {}", e)),
        };
        if file.metadata().unwrap().len() == 0 {
            return Err("Empty file".to_string());
        }
        let mut reader = BufReader::new(file);
        ByteCode::do_transpile(&mut reader)
    }
    pub fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }
    pub fn get_data_size(&self) -> usize {
        self.data_size
    }
    fn new() -> Self {
        ByteCode {
            instructions: Vec::new(),
            data_size: 0,
        }
    }
    fn add_instruction(&mut self, ctx: &mut Context, s_instr: &str) -> Result<(), String> {
        match Instruction::parse(ctx, s_instr) {
            Ok(instr) => {
                self.instructions.push(instr);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    fn do_transpile<R: BufRead>(reader: &mut R) -> Result<Self, String> {
        let mut program = ByteCode::new();
        let mut ctx = Context::new();
        for (index, line) in reader.lines().enumerate() {
            let ln = match line {
                Ok(line) => line,
                Err(e) => return Err(format!("Error reading line {}: {}", index + 1, e)),
            };
            if ln.is_empty() {
                continue;
            }
            ctx.line_number = index + 1;
            if let Err(e) = program.add_instruction(&mut ctx, &ln) {
                return Err(format!("Transpilation error at line {}: {}", index + 1, e));
            }
        }
        if program.instructions.is_empty() {
            return Err("Empty program".to_string());
        }
        program.data_size = ctx.data_size();
        return Ok(program);
    }
}

impl Visitor<Context> for Instruction {
    fn visit_load(&mut self, ctx: &mut Context) -> Result<(), String> {
        if ctx.args_len() != 1 {
            return Err(format!("expected 1 argument, got {}", ctx.args_len()));
        }
        let arg1 = match ctx.get_arg(0).unwrap().parse::<Value>() {
            Ok(v) => v,
            Err(e) => return Err(format!("Error on line {}: {}", ctx.line_number, e)),
        };
        self.opcode = Some(OpCodes::LOAD);
        self.args = Some(vec![arg1]);
        Ok(())
    }
    fn visit_wrt(&mut self, ctx: &mut Context) -> Result<(), String> {
        if ctx.args_len() != 1 {
            return Err(format!("expected 1 argument, got {}", ctx.args_len()));
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").expect("Invalid regex");
        }
        let arg0 = ctx.get_arg(0).unwrap().clone();
        if !RE.is_match(&arg0) {
            return Err(format!("invalid variable name {}", arg0));
        }
        let var0 = ctx.get_var(&arg0);
        let address = match var0 {
            Some(v) => v,
            None => ctx.add_var(&arg0),
        };
        self.opcode = Some(OpCodes::WRT);
        self.args = Some(vec![address]);
        Ok(())
    }
    fn visit_read(&mut self, ctx: &mut Context) -> Result<(), String> {
        if ctx.args_len() != 1 {
            return Err(format!("expected 1 argument, got {}", ctx.args_len()));
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
        }
        let arg0 = ctx.get_arg(0).unwrap();
        if !RE.is_match(arg0) {
            return Err(format!("invalid variable name {}", arg0));
        }
        if !ctx.has_var(arg0) {
            return Err(format!("undeclared variable {}", arg0));
        }
        self.opcode = Some(OpCodes::READ);
        self.args = Some(vec![ctx.get_var(arg0).unwrap()]);
        Ok(())
    }
    fn visit_add(&mut self, ctx: &mut Context) -> Result<(), String> {
        if ctx.args_len() != 0 {
            return Err(format!("expected 0 arguments, got {}", ctx.args_len()));
        }
        self.opcode = Some(OpCodes::ADD);
        self.args = None;
        Ok(())
    }
    fn visit_mult(&mut self, ctx: &mut Context) -> Result<(), String> {
        if ctx.args_len() != 0 {
            return Err(format!("expected 0 arguments, got {}", ctx.args_len()));
        }
        self.opcode = Some(OpCodes::MULT);
        self.args = None;
        Ok(())
    }
    fn visit_rtn(&mut self, ctx: &mut Context) -> Result<(), String> {
        if ctx.args_len() != 0 {
            return Err(format!("expected 0 arguments, got {}", ctx.args_len()));
        }
        self.opcode = Some(OpCodes::RTN);
        self.args = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl ToString for Instruction {
        fn to_string(&self) -> String {
            let args = match &self.args {
                Some(data) => data
                    .iter()
                    .map(|arg| format!("0x{:02x}", arg))
                    .collect::<Vec<String>>()
                    .join(" "),
                None => "".to_string(),
            };
            format!(
                "0x{:02x} {}",
                self.opcode.expect("Illegal instruction: empty opcode") as u8,
                args
            )
            .trim()
            .to_string()
        }
    }

    #[test]
    fn transpilation_error() {
        let code = concat!("READ_VAR x\n", "RETURN_VALUE");
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err("Transpilation error at line 1: undeclared variable x".to_string())
        );
    }

    #[test]
    fn add_load_instruction() {
        let code = "LOAD_VAL 1";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(bytecode.instructions.len(), 1);
        assert_eq!(bytecode.instructions[0].to_string(), "0x01 0x01");
    }
    #[test]
    fn add_load_invalid_arg_num_instruction() {
        let code = "LOAD_VAL 1 1";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err("Transpilation error at line 1: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn add_wrt_instruction() {
        let code = "WRITE_VAR x";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(bytecode.instructions.len(), 1);
        assert_eq!(bytecode.instructions[0].to_string(), "0x02 0x00");
    }

    #[test]
    fn add_wrt_instruction_second_var() {
        let code = "WRITE_VAR x\nWRITE_VAR y";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(bytecode.instructions.len(), 2);
        assert_eq!(bytecode.instructions[0].to_string(), "0x02 0x00");
        assert_eq!(bytecode.instructions[1].to_string(), "0x02 0x01");
    }

    #[test]
    fn add_wrt_invalid_var_instruction() {
        let invalid_var = "%bad-var";
        let code = format!("WRITE_VAR {}", invalid_var);
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err(format!(
                "Transpilation error at line 1: invalid variable name {}",
                invalid_var
            ))
        );
    }

    #[test]
    fn add_wrt_invalid_arg_num_instruction() {
        let code = "WRITE_VAR x y";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err("Transpilation error at line 1: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn add_read_instruction() {
        let code = "WRITE_VAR x\nREAD_VAR x";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(bytecode.instructions.len(), 2);
        assert_eq!(bytecode.instructions[0].to_string(), "0x02 0x00");
        assert_eq!(bytecode.instructions[1].to_string(), "0x03 0x00");
    }

    #[test]
    fn add_read_undeclared_instruction() {
        let undeclared_var = "x";
        let code = format!("READ_VAR {}", undeclared_var);
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err(format!(
                "Transpilation error at line 1: undeclared variable {}",
                undeclared_var
            ))
        );
    }

    #[test]
    fn add_read_invalid_var_instruction() {
        let invalid_var = "%bad-var";
        let code = format!("READ_VAR {}", invalid_var);
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err(format!(
                "Transpilation error at line 1: invalid variable name {}",
                invalid_var
            ))
        );
    }

    #[test]
    fn add_read_invalid_arg_num_instruction() {
        let code = "READ_VAR x y";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err("Transpilation error at line 1: expected 1 argument, got 2".to_string())
        );
    }

    #[test]
    fn add_add_instruction() {
        let code = "ADD";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(bytecode.instructions.len(), 1);
        assert_eq!(bytecode.instructions[0].to_string(), "0x04");
    }

    #[test]
    fn add_add_invalid_arg_num_instruction() {
        let code = "ADD 1";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err("Transpilation error at line 1: expected 0 arguments, got 1".to_string())
        );
    }

    #[test]
    fn add_mult_instruction() {
        let code = "MULTIPLY";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(bytecode.instructions.len(), 1);
        assert_eq!(bytecode.instructions[0].to_string(), "0x05");
    }

    #[test]
    fn add_mult_invalid_arg_num_instruction() {
        let code = "MULTIPLY 1";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err("Transpilation error at line 1: expected 0 arguments, got 1".to_string())
        );
    }

    #[test]
    fn add_rtn_instruction() {
        let code = "RETURN_VALUE";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert!(result.is_ok());
        let bytecode = result.ok().unwrap();
        assert_eq!(bytecode.instructions.len(), 1);
        assert_eq!(bytecode.instructions[0].to_string(), "0x06");
    }

    #[test]
    fn add_rtn_invalid_arg_num_instruction() {
        let code = "RETURN_VALUE 1";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err("Transpilation error at line 1: expected 0 arguments, got 1".to_string())
        );
    }

    #[test]
    fn add_unknown_instruction() {
        let code = "NONEXISTENT_OP";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(
            result,
            Err(format!(
                "Transpilation error at line 1: unknown opcode: {}",
                code
            ))
        );
    }

    #[test]
    fn add_empty_instruction() {
        let code = "";
        let mut reader = BufReader::new(code.as_bytes());
        let result = ByteCode::do_transpile(&mut reader);
        assert_eq!(result, Err("Empty program".to_string()));
    }
}
