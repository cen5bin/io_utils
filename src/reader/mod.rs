mod file_reader;
pub use self::file_reader::FileReader;

mod buffered_reader;
pub use self::buffered_reader::BufferedReader;

pub trait Reader {
    fn read(&mut self) -> Option<&str>;
    fn read_line(&mut self) -> Option<&str>;
}