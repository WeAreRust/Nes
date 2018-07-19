#![feature(proc_macro)]

extern crate heck;
extern crate proc_macro2;
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::{Diagnostic, TokenStream};
use syn::{parse2, DeriveInput};

mod instruction;

type DeriveFn = fn(DeriveInput) -> Result<proc_macro2::TokenStream, Diagnostic>;

#[proc_macro_derive(Instruction, attributes(opcode, cycles))]
pub fn instruction_derive(tokens: TokenStream) -> TokenStream {
    expand_derive(tokens, instruction::derive)
}

fn expand_derive(tokens: TokenStream, derive: DeriveFn) -> TokenStream {
    let item = parse2(tokens.into()).unwrap();
    match derive(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => handle_derive_err(err),
    }
}

fn handle_derive_err(err: Diagnostic) -> TokenStream {
    err.emit();
    "".parse().unwrap()
}
