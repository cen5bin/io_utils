

mod file_stream;
pub use self::file_stream::{FileOutputStream, FileInputStream};

mod buffered_stream;

pub trait InputStream {
    fn read(&mut self, buf: &mut [u8]) -> usize;
    fn read_to(&mut self, buf: &mut [u8], off: usize, len: usize) -> usize;
}

pub trait OutputStream {
    fn write(&mut self, buf: &[u8]);
    fn write_slice(&mut self, buf: &[u8], off: usize, len: usize);
    fn flush(&mut self);
}