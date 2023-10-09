use std::io;

use wasm_bridge::{component::Linker, Result, StoreContextMut};

use super::{stream, WasiView};

pub(crate) fn add_to_linker<T: 'static + WasiView>(linker: &mut Linker<T>) -> Result<()> {
    linker.instance("wasi:cli/terminal-input")?.func_wrap(
        "drop-terminal-input",
        |_data: StoreContextMut<T>, (_this,): (u32,)| -> Result<()> { Ok(()) },
    )?;

    linker.instance("wasi:cli/terminal-output")?.func_wrap(
        "drop-terminal-output",
        |_data: StoreContextMut<T>, (_this,): (u32,)| -> Result<()> { Ok(()) },
    )?;

    linker.instance("wasi:cli/terminal-stdin")?.func_wrap(
        "get-terminal-stdin",
        |_data: StoreContextMut<T>, (): ()| -> Result<Option<u32>> { Ok(None) },
    )?;

    linker.instance("wasi:cli/terminal-stdout")?.func_wrap(
        "get-terminal-stdout",
        |_data: StoreContextMut<T>, (): ()| -> Result<Option<u32>> { Ok(None) },
    )?;

    linker.instance("wasi:cli/terminal-stderr")?.func_wrap(
        "get-terminal-stderr",
        |_data: StoreContextMut<T>, (): ()| -> Result<Option<u32>> { Ok(None) },
    )?;

    Ok(())
}
