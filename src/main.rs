use clap::{arg, Command};
use morango::interpret;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Morango interpreter")
        .version("0.1.0")
        .author("Alex Mikhalevich <alex@mikhalevich.com>")
        .about("Interpreter for the toy Morango language")
        .arg(arg!(
            -f --file <FILE> "Source code to interpret"
        ))
        .get_matches();
    let source_file = matches
        .value_of("file")
        .expect("You should specify a file to interpret");
    let result = interpret(source_file)?;
    println!("{}", result.unwrap());
    Ok(())
}
