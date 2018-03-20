use super::super::stream::{OutputStream, FileOutputStream};
use super::Writer;


pub struct FileWriter {
    fos: FileOutputStream,
}

impl Writer for FileWriter {
    fn write(&mut self, data: &String) {
        self.fos.write(data.as_bytes());
    }

    fn write_str(&mut self, data: &str) {
        self.fos.write(data.as_bytes());
    }
}
