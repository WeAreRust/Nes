use heck::SnakeCase;
use proc_macro::Diagnostic;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use syn::{DeriveInput, Lit, Meta::NameValue};

// TODO(joshleeb): Proper error handling.
// Could even update flags from a custom attribute.
pub fn derive(item: DeriveInput) -> Result<TokenStream, Diagnostic> {
    let ident = item.ident;
    let lower_ident = Ident::new(&format!("{}", ident).to_snake_case(), Span::call_site());

    let mut attrs = item.attrs.iter().map(|a| a.interpret_meta().unwrap());

    let opcode = match attrs.find(|a| a.name() == "opcode") {
        Some(NameValue(value)) => value.lit,
        _ => unimplemented!(),
    };

    let cycles = match attrs.find(|a| a.name() == "cycles") {
        Some(NameValue(value)) => value.lit,
        _ => unimplemented!(),
    };

    let extra_cycles = match attrs.find(|a| a.name() == "extra_cycles") {
        Some(NameValue(value)) => value.lit,
        _ => Lit::new(Literal::usize_unsuffixed(0)),
    };

    Ok(quote! {
        impl Instruction for #ident {
            const OPCODE: u8 = #opcode;
            const CYCLES: usize = #cycles;
            const PAGE_BOUNDARY_EXTRA_CYCLES: usize = #extra_cycles;

            fn exec(core: &mut Core, memory: &mut Memory) {
                #lower_ident(core, memory)
            }
        }
    })
}
