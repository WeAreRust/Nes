use heck::SnakeCase;
use proc_macro::Diagnostic;
use proc_macro2::{Ident, Span, TokenStream};
use syn::{DeriveInput, Meta::NameValue};

// TODO(joshleeb): Proper error handling.
// TODO(joshleeb): Cleanup big time!!
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

  let extra_cycles = match attrs.find(|a| a.name() == "page_boundary_extra_cycle") {
    Some(NameValue(_)) => true,
    _ => false,
  };

  Ok(quote! {
      impl Execute for #ident {
          const OPCODE: u8 = #opcode;
          const CYCLES: usize = #cycles;
          const PAGE_BOUNDARY_EXTRA_CYCLES: bool = #extra_cycles;

          fn exec<T>(core: &mut ::cpu::Core, memory: &mut T)
          where
              T: ::memory::ReadAddr + ::memory::WriteAddr,
          {
              #lower_ident(core, memory)
          }
      }
  })
}
