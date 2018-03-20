

#[inline]
pub fn memchr(buf: &[u8], x: u8) -> Option<usize> {
    for i in 0..buf.len() {
        if buf[i] == x {
            return Some(i);
        }
    }
    None
}

