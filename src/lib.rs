use std::fmt;
use std::error::Error;
use std::io::Read;
use std::io;

#[derive(Debug)]
pub enum PossibleErrors {
    IoError(io::Error),
    MagicNumber(WrongMagicNumber)
}

#[derive(Debug)]
pub struct WrongMagicNumber {
    expected: Vec<u8>,
    read: Vec<u8>,
}

impl Error for WrongMagicNumber {
    fn description(&self) -> &str {
        "A Magic Number check Failed"
    }
}

impl fmt::Display for WrongMagicNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut expected_string_hex = String::new();
        let mut count = 0;
        for byte in self.expected.clone() {
            if count < self.expected.len() {
                expected_string_hex.push_str(&format!("{:X},", byte))
            } else {
                expected_string_hex.push_str(&format!("{:X}", byte))
            };
            count += 1;
        }
        let expected_string = String::from_utf8(self.expected.clone()).unwrap_or(expected_string_hex);
        let mut read_string_hex = String::new();
        let mut count = 0;
        for byte in self.read.clone() {
            if count < self.read.len() {
                read_string_hex.push_str(&format!("{:X},", byte))
            } else {
                read_string_hex.push_str(&format!("{:X},", byte))
            };
            count += 1;
        }
        let read_string = String::from_utf8(self.read.clone()).unwrap_or(read_string_hex);
        write!(
            f,
            "Incorrect Magic Number: Expected '{}', Read '{}'",
            expected_string, read_string
        )
    }
}

pub fn check_magic_number<R: Read>(
    reader: &mut R,
    magic_number: Vec<u8>,
) -> Result<(), PossibleErrors> {
    let mut read = magic_number.clone();
    match reader.read_exact(&mut read) {
        Ok(_) => {},
        Err(e) => return Err(PossibleErrors::IoError(e))
    }
    if read != magic_number {
        Err(PossibleErrors::MagicNumber(WrongMagicNumber { expected: magic_number, read }))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use check_magic_number;
    use PossibleErrors;
    #[test]
    fn check_valid() {
        let ref mut reader = Cursor::new(vec![b'T', b'E', b'S', b'T']);
        check_magic_number(reader, vec![b'T', b'E', b'S', b'T']).unwrap();
    }

    #[test]
    fn check_invalid() {
        let ref mut reader = Cursor::new(vec![b'T', b'E', b'S', b'T']);
        match check_magic_number(reader, vec![b'N', b'E', b'S', b'T']) {
            Ok(_) => panic!("Expected an error"),
            Err(PossibleErrors::MagicNumber(_)) => {},
            Err(_) => panic!("Unexpected error")
        }
    }
}