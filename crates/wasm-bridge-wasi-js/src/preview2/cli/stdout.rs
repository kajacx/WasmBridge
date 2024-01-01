use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use crate::preview2::{WasiView, STDOUT_IDENT};

wasm_bridge::component::bindgen!({
    path: "src/preview2/wits/stdout.wit",
    world: "exports"
});

impl<T: WasiView> wasi::cli::stdout::Host for T {
    fn get_stdout(&mut self) -> Result<u32> {
        Ok(STDOUT_IDENT)
    }
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    Exports::add_to_linker(linker, |d| d)
}