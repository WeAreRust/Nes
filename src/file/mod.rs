mod ines;

#[derive(PartialEq, Debug)]
pub enum Format {
  INES,
}

pub struct Cartridge {

}

#[derive(PartialEq, Debug)]
pub struct UnknownFormat {}

pub fn detect_format(data: &[u8]) -> Result<Format, UnknownFormat> {
  if ines::check_format(data) {
    Ok(Format::INES)
  } else {
    Err(UnknownFormat{})
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_detect_format_ines() {
        let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
        assert_eq!(detect_format(&data), Ok(Format::INES));
    }
}
