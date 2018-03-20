use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::Error;

use std::slice;

use super::{InputStream, OutputStream};



pub struct FileOutputStream {
    f: File,
}

impl FileOutputStream {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let f = File::create(path)?;
        let ret = FileOutputStream {
            f,
        };
        Ok(ret)
    }
}

impl OutputStream for FileOutputStream {
    fn write(&mut self, buf: &[u8]) {
        self.f.write_all(buf).unwrap();
    }

    fn write_slice(&mut self, buf: &[u8], off: usize, len: usize) {
        let slice = unsafe { slice::from_raw_parts(buf.as_ptr().offset(off as isize), len) };
        self.f.write_all(slice).unwrap();
    }

    fn flush(&mut self) {
    }
}

pub struct FileInputStream {
    f: File,
}

impl FileInputStream {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error>  {
        let f = File::open(path)?;
        let ret = FileInputStream {
            f,
        };
        Ok(ret)
    }
}

impl InputStream for FileInputStream {
    fn read(&mut self, buf: &mut [u8]) -> usize {
        self.f.read(buf).unwrap()
    }

    fn read_to(&mut self, buf: &mut [u8], off: usize, len: usize) -> usize {
        let data = unsafe {
            slice::from_raw_parts_mut(buf.as_mut_ptr().offset(off as isize), len)
        };
        self.f.read(data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::fs::*;
    use std::str;

    #[test]
    fn test_file_stream() {
        let test_file = "test_file_stream";
        let mut fos = FileOutputStream::new(test_file).unwrap();
        let data = "asdasdasdasdas";
        let len = data.as_bytes().len();
        fos.write(data.as_bytes());
        let mut fis = FileInputStream::new(test_file).unwrap();
        let mut buf = vec![0u8; len];
        let size = fis.read(buf.as_mut());
        assert_eq!(size, len);
        assert_eq!(str::from_utf8(&buf).unwrap(), data);

        let mut fis = FileInputStream::new(test_file).unwrap();
        for i in 0..len {
            fis.read_to(buf.as_mut(), i, 1);
        }

        assert_eq!(str::from_utf8(&buf).unwrap(), data);

        rm(test_file);
    }

}
