use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must supply ROM filename.");
    }

    let filename = &args[1];
    println!("Loading ROM: {}", filename);
    let mut f = File::open(filename).expect("File not found");
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data).unwrap();

    // TODO(toby): parse the file content
    // let cartridge = cartridge::parse_rom_file(data[..]);

    println!("NES Emulator");
}
