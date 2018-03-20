use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::mem::size_of;

pub struct BinaryFile {
    f: File,
    val_buf: Vec<u8>,
}

macro_rules! read_val_func {
        ($func:tt, $ty:ty) => {
            #[inline]
            pub fn $func(&mut self) -> Option<$ty> {
                let size = self.f.read(self.val_buf[0..size_of::<$ty>()].as_mut()).unwrap();
                if size < size_of::<$ty>() {
                    None
                } else {
                    Some(0)
                }
            }
        };
}


impl BinaryFile {
    pub fn create<P: AsRef<Path>>(path: P) -> Self {
        let f = File::create(path).unwrap();
        BinaryFile {
            f,
            val_buf: vec![0u8; 8],
        }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        let f = File::open(path).unwrap();
        BinaryFile {
            f,
            val_buf: vec![0u8; 8],
        }
    }

    pub fn test(&mut self) {
        self.f.read(self.val_buf[0..size_of::<i32>()].as_mut());
    }

    read_val_func!(read_i16, i16);
    read_val_func!(read_i32, i32);
    read_val_func!(read_i64, i64);




    
//    pub fn read_i32(&mut self) -> Option<i32> {
//
//    }

}