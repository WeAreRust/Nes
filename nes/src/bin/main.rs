extern crate nes;

use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use nes::clock::Clock;
use nes::clock::Processor;

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

    let mut clock = Clock::new();
    let mut cpu = nes::cpu::Core::default();
    let mut ram = nes::memory::block::BlockMemory::with_size(0x0800);
    let mut bus = nes::bus::Bus::new(&mut cartridge, &mut ram);
    let mut cpu_interval: u8 = 0;
    let mut ppu_interval: u8 = 0;
    let mut total_cycles: u64 = 0;
    let mut report_cycle: u32 = 0;
    let start = Instant::now();

    cpu.reset(&bus);
    loop {
        clock.cycle();
        cpu_interval += 1;
        ppu_interval += 1;

        if cpu_interval == nes::clock::CPU_PERIOD {
            cpu_interval = 0;
            cpu.cycle(&mut bus);
        }

        if ppu_interval == nes::clock::PPU_FREQUENCY {
            ppu_interval = 0;
            // TODO: ppu.cycle()
        }

        total_cycles += 1;
        report_cycle += 1;
        if report_cycle == nes::clock::MASTER_FREQUENCY {
            report_cycle = 0;
            println!("Total cycles: {}; time: {:?} secs", total_cycles, Instant::now().duration_since(start));
        }
    }
}
