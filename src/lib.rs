use std::fmt;
use std::error::Error;
use std::io::Read;
use std::io;

#[derive(Debug)]
pub enum MagicNumberCheckError {
    IoError(io::Error),
    MagicNumber(WrongMagicNumber)
}

impl Error for MagicNumberCheckError {
    fn description(&self) -> &str {
        match *self {
            MagicNumberCheckError::IoError(ref e) => e.description(),
            MagicNumberCheckError::MagicNumber(ref e) => e.description()
        }
    }
}

impl fmt::Display for MagicNumberCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MagicNumberCheckError::IoError(ref e) => e.fmt(f),
            MagicNumberCheckError::MagicNumber(ref e) => e.fmt(f)
        }
    }
}

impl From<io::Error> for MagicNumberCheckError {
    fn from(e: io::Error) -> MagicNumberCheckError {
        MagicNumberCheckError::IoError(e)
    }
}

impl From<WrongMagicNumber> for MagicNumberCheckError {
    fn from(e: WrongMagicNumber) -> MagicNumberCheckError {
        MagicNumberCheckError::MagicNumber(e)
    }
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
        let expected_string = String::from_utf8(self.expected.clone()).unwrap_or(format!("{:X?}", self.expected));
        let read_string = String::from_utf8(self.read.clone()).unwrap_or(format!("{:X?}", self.read));
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
) -> Result<(), MagicNumberCheckError>{
    let mut read = magic_number.clone();
    reader.read_exact(&mut read)?;
    if read != magic_number {
        Err(MagicNumberCheckError::MagicNumber(WrongMagicNumber { expected: magic_number, read }))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use check_magic_number;
    use MagicNumberCheckError;
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
            Err(MagicNumberCheckError::MagicNumber(_)) => {},
            Err(_) => panic!("Unexpected error")
        }
    }
}