extern crate nes;

use std::env;
use std::fs::File;
use std::io::Read;

use nes::console::Console;

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
  let mut cartridge = nes::cartridge::parse_rom_file(&data).unwrap();
  print!("PRG ROM DUMP");
  for i in 0x8000..0xC000 {
    if (i - 0x8000) % 0x10 == 0 {
      println!();
      print!("${:04X}", i);
    }
    print!(" {:02x}", cartridge.mapper.read_addr(i));
  }
  println!();

  println!("Cartridge loaded.");

  let mut console = Console {
    cartridge: &mut cartridge,
  };
  console.run();
}
