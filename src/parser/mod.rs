use crate::bytecode::{instruction::Context, ByteCode};
use std::fs::File;
use std::io::{BufRead, BufReader};


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::tempfile;

    impl ToString for ByteCode {
        fn to_string(&self) -> String {
            self.instructions
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        }
    }

    #[test]
    fn transpile_simple_file() {
        let mut file = tempfile().unwrap();
        let code = concat!(
            "LOAD_VAL 1\n",
            "WRITE_VAR x\n",
            "LOAD_VAL 2\n",
            "WRITE_VAR y\n",
            "READ_VAR x\n",
            "LOAD_VAL 1\n",
            "ADD\n",
            "READ_VAR y\n",
            "MULTIPLY\n",
            "RETURN_VALUE"
        );
        writeln!(&mut file, "{}", code).ok();
        file.seek(SeekFrom::Start(0)).ok();

        let program = concat!(
            "0x01 0x01 ",
            "0x02 0x00 ",
            "0x01 0x02 ",
            "0x02 0x01 ",
            "0x03 0x00 ",
            "0x01 0x01 ",
            "0x04 ",
            "0x03 0x01 ",
            "0x05 ",
            "0x06"
        );

        let result = do_transpile(&file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), program);
    }

}
