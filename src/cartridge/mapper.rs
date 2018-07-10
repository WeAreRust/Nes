#[derive(PartialEq, Debug)]
pub enum Mapper {
    NROM, // No mapper
    NintendoMMC1,
    CNROMSwitch,
    INESMapper211, // https://wiki.nesdev.com/w/index.php/INES_Mapper_211
}
