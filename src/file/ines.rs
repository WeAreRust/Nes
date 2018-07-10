const INES_HEADER: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];

pub fn check_format(data: &[u8]) -> bool {
    data[0..4] == INES_HEADER
}

fn count_prg_rom_banks(data: &[u8]) -> u8 {
    data[4]
}

fn count_chr_rom_banks(data: &[u8]) -> u8 {
    data[5]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_check_format() {
        let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
        assert!(check_format(&data));
    }

    #[test]
    pub fn test_count_prg_rom_banks() {
        let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
        assert_eq!(count_prg_rom_banks(&data), 16u8);
    }

    #[test]
    pub fn test_count_chr_rom_banks() {
        let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
        assert_eq!(count_chr_rom_banks(&data), 32u8);
    }
}
