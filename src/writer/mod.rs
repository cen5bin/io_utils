mod file_writer;
pub use self::file_writer::FileWriter;

mod buffered_writer;
pub use self::buffered_writer::BufferedWriter;

pub trait Writer {
    fn write(&mut self, data: &String);
    fn write_str(&mut self, data: &str);
}