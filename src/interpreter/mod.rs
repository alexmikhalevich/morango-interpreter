mod interpreter;
mod stack;

use crate::bytecode::ByteCode;
use crate::config::Value;
use interpreter::Interpreter;

pub fn interpret(program: ByteCode) -> Result<Option<Value>, String> {
    let mut interpreter = Interpreter::new(program);
    interpreter.interpret()
}
