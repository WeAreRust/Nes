# Nes

NES Emulator.

## Notes

- Currently, we are using the [asm6502](https://crates.io/crates/asm6502) crate for assembling the 6502 CPU assebmly code set into machine code. This is used to make the tests more readable. In the future (mainly for fun), we may want to write our own assembler and disassembler for the 6502 CPU (asmdi-6502?)
