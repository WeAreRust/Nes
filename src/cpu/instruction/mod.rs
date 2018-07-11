#[macro_export]
macro_rules! nes_asm {
    ($e:expr) => {{
        let mut buf = vec![];
        ::asm6502::assemble($e.as_bytes(), &mut buf).unwrap();
        buf
    }};
}

mod jmp;
