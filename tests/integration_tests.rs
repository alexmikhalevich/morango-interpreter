use morango::interpret;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
pub fn test_simple_program() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let code = concat!(
        "LOAD_VAL 1\n",
        "WRITE_VAR x\n",
        "\n",
        "LOAD_VAL 2\n",
        "WRITE_VAR y\n",
        "\n",
        "READ_VAR x\n",
        "LOAD_VAL 1\n",
        "ADD\n",
        "READ_VAR y\n",
        "MULTIPLY\n",
        "RETURN_VALUE\n",
        "\n",
    );
    write!(file, "{}", code).expect("Failed to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Failed to convert temp file path to string"),
    );
    assert!(result.is_ok());
    assert!(result.as_ref().ok().is_some());
    assert_eq!(result.ok().unwrap().unwrap() as u16, 4);
}

#[test]
pub fn test_early_return() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let code = concat!(
        "LOAD_VAL 1\n",
        "WRITE_VAR x\n",
        "\n",
        "LOAD_VAL 2\n",
        "WRITE_VAR y\n",
        "\n",
        "READ_VAR x\n",
        "LOAD_VAL 1\n",
        "ADD\n",
        "RETURN_VALUE\n",
        "READ_VAR y\n",
        "MULTIPLY\n",
        "\n",
    );
    write!(file, "{}", code).expect("Failed to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Failed to convert temp file path to string"),
    );
    assert!(result.is_ok());
    assert!(result.as_ref().ok().is_some());
    assert_eq!(result.ok().unwrap().unwrap() as u16, 2);
}

#[test]
pub fn test_no_return() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let code = concat!("LOAD_VAL 1\n", "LOAD_VAL 2\n", "ADD\n",);
    write!(file, "{}", code).expect("Failed to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Failed to convert temp file path to string"),
    );
    assert!(result.is_ok());
    assert!(result.as_ref().ok().unwrap().is_none());
}

#[test]
pub fn test_empty_code() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let code = "".to_string();
    write!(file, "{}", code).expect("Failed to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Failed to convert temp file path to string"),
    );
    assert_eq!(result, Err("Empty file".to_string()));
}

#[test]
pub fn test_invalid_code_undeclared_var() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let code = "READ_VAR x".to_string();
    write!(file, "{}", code).expect("Failed to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Failed to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err("Transpilation error at line 1: undeclared variable x".to_string())
    );
}

#[test]
pub fn test_invalid_code_empty_stack() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let code = "ADD".to_string();
    write!(file, "{}", code).expect("Failed to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Failed to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err(
            "Runtime error: unable to process current instruction, ip = 0x00: no value on stack"
                .to_string()
        )
    );
}

#[test]
pub fn test_invalid_code_no_args() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let code = "READ_VAR".to_string();
    write!(file, "{}", code).expect("Failed to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Failed to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err("Transpilation error at line 1: expected 1 argument, got 0".to_string())
    );
}

#[test]
pub fn test_invalid_code_excessive_args() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let code = "WRITE_VAR x x".to_string();
    write!(file, "{}", code).expect("Failed to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Failed to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err("Transpilation error at line 1: expected 1 argument, got 2".to_string())
    );
}

#[test]
pub fn test_invalid_code_invalid_variable_name() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let code = "WRITE_VAR .x".to_string();
    write!(file, "{}", code).expect("Failed to write to temp file");

    let result = interpret(
        file.path()
            .to_str()
            .expect("Failed to convert temp file path to string"),
    );
    assert_eq!(
        result,
        Err("Transpilation error at line 1: invalid variable name .x".to_string())
    );
}
