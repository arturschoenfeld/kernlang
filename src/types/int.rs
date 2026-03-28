use std::error::Error;

pub fn parse_int(text: &[u8]) -> Result<i128, Box<dyn Error>> {
    if text.starts_with(b"0b") || text.starts_with(b"0B") {
        Ok(i128::from_str_radix(&std::str::from_utf8(&text[2..])?, 2)?)
    } else if text.starts_with(b"0o") || text.starts_with(b"0O") {
        Ok(i128::from_str_radix(&std::str::from_utf8(&text[2..])?, 8)?)
    } else if text.starts_with(b"0x") || text.starts_with(b"0X") {
        Ok(i128::from_str_radix(&std::str::from_utf8(&text[2..])?, 16)?)
    } else {
        Ok(std::str::from_utf8(text)?.parse::<i128>()?)
    }
}