use super::context::Context;
use crate::config::{OpCodes, Value, Visitor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Option<OpCodes>,
    pub args: Option<Vec<Value>>,
}

impl Instruction {
    pub fn parse(ctx: &mut Context, s: &str) -> Result<Self, String> {
        if s.is_empty() {
            return Err("Empty instruction".to_string());
        }
        let s_split = s
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        ctx.set_args(s_split[1..].to_vec());
        let mut instr = Instruction {
            opcode: None,
            args: None,
        };

        match s_split[0].as_str() {
            "LOAD_VAL" => instr.visit_load(ctx),
            "WRITE_VAR" => instr.visit_wrt(ctx),
            "READ_VAR" => instr.visit_read(ctx),
            "ADD" => instr.visit_add(ctx),
            "MULTIPLY" => instr.visit_mult(ctx),
            "RETURN_VALUE" => instr.visit_rtn(ctx),
            "GOTO" => instr.visit_goto(ctx),
            "TEST_EQ" => instr.visit_test_eq(ctx),
            "TEST_GT" => instr.visit_test_gt(ctx),
            "TEST_LT" => instr.visit_test_lt(ctx),
            "DUP" => instr.visit_dup(ctx),
            "POP" => instr.visit_pop(ctx),
            other => {
                if Context::is_label(other) {
                    if ctx.has_label(&s_split[0]) {
                        Err(format!("duplicated label: {}", s_split[0]))?;
                    }
                    ctx.add_label(&s_split[0], ctx.instruction_number);
                } else {
                    Err(format!("unknown instruction: {}", &s_split[0]))?;
                }
                return Ok(instr);
            }
        }?;
        Ok(instr)
    }
}
