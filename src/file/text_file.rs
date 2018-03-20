use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

use ::string_util::*;

pub struct TextWriter {
    f: File,
}

impl TextWriter {

    pub fn create<P: AsRef<Path>>(path: P) -> Self {
        TextWriter {
            f: File::create(path).unwrap(),
        }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        TextWriter {
            f: File::open(path).unwrap(),
        }
    }

    #[inline]
    pub fn write_str(&mut self, data: &str) {
        self.f.write_all(data.as_bytes()).unwrap();
    }

    #[inline]
    pub fn write(&mut self, data: &String) {
        self.f.write_all(data.as_bytes()).unwrap();
    }

    #[inline]
    pub fn write_val<T: ToString + Copy>(&mut self, data: T) {
        self.write(&data.to_string());
    }

}

pub struct TextReader {
    f: File,
    buf: Vec<u8>,
    pos: usize,
    end: usize,
}

impl TextReader {

    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        let f = File::open(path).unwrap();
        TextReader {
            f,
            buf: vec![0u8; 8192],
            pos: 0,
            end: 0,
        }
    }

    /// Read all bytes until a newline is reached, and append them to the provided buf.
    /// If successful, this function will return the total number of bytes read.
    /// If 0 returned indicates that it has reached EOF
    pub fn read_line(&mut self, line: &mut String) -> usize {
        line.clear();
        {
            let str_vec = unsafe { line.as_mut_vec() };
            let mut read_size = 0;
            loop {
                for i in self.pos..self.end {
                    if self.buf[i] == ('\r' as u8) || self.buf[i] == ('\n' as u8) {
                        str_vec.extend_from_slice(self.buf[self.pos..i].as_ref());
                        let tmp = self.pos;

                        self.pos = i;
                        if self.buf[self.pos] == ('\r' as u8) {
                            self.pos += 1;
                        }
                        if self.buf[self.pos] == ('\n' as u8) {
                            self.pos += 1;
                        }
                        read_size += self.pos - tmp;
                        return read_size;
                    }
                }
                str_vec.extend_from_slice(self.buf[self.pos..self.end].as_ref());
                if self.fill_buf() == 0 {
                    break;
                }
            }
        }
        return line.len();
    }

    /// Read all bytes until a blank character is reached, and append them to the provided buf.
    /// An Empty buf returned indicates that is has reached EOF.
    pub fn read(&mut self, data: &mut String) {
        data.clear();
        let str_vec = unsafe {
            data.as_mut_vec()
        };
        loop {
            while self.pos < self.end && is_blank_character(self.buf[self.pos]) {
                self.pos += 1;
            }
            for i in self.pos..self.end {
                if is_blank_character(self.buf[i]) {
                    str_vec.extend_from_slice(self.buf[self.pos..i].as_ref());
                    self.pos = i + 1;
                    return;
                }
            }
            str_vec.extend_from_slice(self.buf[self.pos..self.end].as_ref());
            if self.fill_buf() == 0 {
                break;
            }
        }
    }

    #[inline]
    fn fill_buf(&mut self) -> usize {
        self.pos = 0;
        let size = self.f.read(self.buf.as_mut()).unwrap();
        self.end = size;
        size
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use ::fs;

    #[test]
    fn test_text_file() {
        let test_file = "test_text_file";
        let long_str = String::from_utf8(vec!['c' as u8; 1048577]).unwrap();
        {
            let mut writer = TextWriter::create(test_file);
            writer.write(&format!("aaaa\n"));
            writer.write(&format!("bbbb\n"));
            writer.write(&format!("\n"));
            writer.write(&long_str);
        }
        {
            let mut reader = TextReader::open(test_file);
            let mut line = String::new();
            let size = reader.read_line(&mut line);
            assert_eq!(size, 5);
            assert_eq!(line, "aaaa");
            let size = reader.read_line(&mut line);
            assert_eq!(size, 5);
            assert_eq!(line, "bbbb");
            let size = reader.read_line(&mut line);
            assert_eq!(size, 1);
            assert_eq!(line, "");
            let size = reader.read_line(&mut line);
            assert_eq!(size, long_str.len());
            assert_eq!(line, long_str);
            let size = reader.read_line(&mut line);
            assert_eq!(size, 0);
        }

        {
            let mut reader = TextReader::open(test_file);
            let mut data = String::new();
            reader.read(&mut data);
            assert_eq!(data, "aaaa");
            reader.read(&mut data);
            assert_eq!(data, "bbbb");
            reader.read(&mut data);
            assert_eq!(data, long_str);
            reader.read(&mut data);
            assert_eq!(data, "");
        }

        fs::rm(test_file);
    }
}