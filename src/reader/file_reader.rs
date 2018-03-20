use super::super::stream::{FileInputStream, InputStream};
use super::Reader;

pub struct FileReader {
    fis: FileInputStream,
}

impl Reader for FileReader {
    fn read(&mut self) -> Option<&str> {
        unimplemented!()
    }

    fn read_line(&mut self) -> Option<&str> {
        unimplemented!()
    }
}