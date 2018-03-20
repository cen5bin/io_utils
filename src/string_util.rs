
#[inline]
pub fn is_blank_character(c: u8) -> bool {
    match c as char {
        ' ' | '\t' | '\r' | '\n'  => true,
        _ => false,
    }
}