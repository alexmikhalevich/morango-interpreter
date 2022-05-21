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
            _ => Err(format!("unknown opcode: {}", &s_split[0])),
        }?;
        Ok(instr)
    }
}
