use super::Writer;


pub struct BufferedWriter<T: Writer> {
    buf: Vec<u8>,
    pos: usize,
    writer: T,
}

impl<T: Writer> BufferedWriter<T> {
    pub fn new(writer: T) -> Self {
        BufferedWriter::with_capacity(writer, 8192)
    }

    pub fn with_capacity(writer: T, capacity: usize) -> Self {
        BufferedWriter {
            buf: vec![0u8; capacity],
            pos: 0,
            writer,
        }
    }
}

impl<T: Writer> Writer for BufferedWriter<T> {
    fn write(&mut self, data: &String) {
        unimplemented!()
    }

    fn write_str(&mut self, data: &str) {
        unimplemented!()
    }
}