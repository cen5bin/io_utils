use std::ops::Drop;
use std::intrinsics::copy_nonoverlapping;

use super::OutputStream;


pub struct BufferedOutputStream<T: OutputStream> {
    buf: Vec<u8>,
    pos: usize,
    output_stream: T,
}

impl<T> BufferedOutputStream<T>
    where T: OutputStream {

    pub fn new(output_stream: T) -> Self {
        BufferedOutputStream {
            buf: vec![0u8; 8192],
            pos: 0,
            output_stream,
        }
    }

    pub fn with_capacity(output_stream: T, capacity: usize) -> Self {
        BufferedOutputStream {
            buf: vec![0u8; capacity],
            pos: 0,
            output_stream,
        }
    }
}

impl<T> OutputStream for BufferedOutputStream<T>
    where T: OutputStream {
    fn write(&mut self, buf: &[u8]) {
        self.write_slice(buf, 0, buf.len());
    }

    fn write_slice(&mut self, buf: &[u8], off: usize, len: usize) {
        if len > self.buf.capacity() - self.pos {
            self.flush();
            if len > self.buf.len() {
                self.output_stream.write_slice(buf, off, len);
                return;
            }
        }
        unsafe {
            let ptr = buf.as_ptr().offset(off as isize);
            let dst = self.buf.as_mut_ptr().offset(self.pos as isize);
            copy_nonoverlapping(ptr, dst, len);
            self.pos += len;
        }
    }

    fn flush(&mut self) {
        self.output_stream.write_slice(&self.buf, 0, self.pos);
        self.pos = 0;
        self.output_stream.flush();
    }
}

impl<T> Drop for BufferedOutputStream<T>
    where T: OutputStream {
    fn drop(&mut self) {
        self.flush();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::{FileOutputStream, InputStream, FileInputStream};
    use ::fs::*;
    use std::str;

    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_buffered_output_stream() {
        let test_file = "test_buffered_output_stream";
        let mut bos = BufferedOutputStream::new(FileOutputStream::new(test_file).unwrap());
        let buf = "abcdefg";
        let mut s = String::new();
        let mut len = 0;
        for _ in 0..10000 {
            len += buf.as_bytes().len();
            bos.write(buf.as_bytes());
            s += buf;
        }
        bos.flush();
        println!("total len: {}", len);

        let mut fis = FileInputStream::new(test_file).unwrap();
        let mut res = vec![0u8; len];
        let mut left = len;
        while left > 0 {
            left -= fis.read_to(&mut res, len - left, left);
        }
        assert_eq!(str::from_utf8(&res).unwrap(), s.as_str());

        rm(test_file);
    }
}