pub fn check_format(data: &[u8]) -> bool {
    data[0..4] == [0x4eu8, 0x45u8, 0x53u8, 0x1au8]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_check_format() {
        let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
        assert!(check_format(&data));
    }
}
