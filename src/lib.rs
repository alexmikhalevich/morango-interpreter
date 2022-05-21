mod bytecode;
mod config;
mod interpreter;

use bytecode::ByteCode;
use config::Value;
use interpreter::interpret as do_interpret;

pub fn interpret(source_file: &str) -> Result<Option<Value>, String> {
    let bytecode = ByteCode::transpile(source_file)?;
    do_interpret(bytecode)
}
