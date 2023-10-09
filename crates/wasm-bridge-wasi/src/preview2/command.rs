use crate::preview2::{clocks, WasiView};
use wasm_bridge::{component::Linker, Result, StoreContextMut};

use super::{environment, filesystem, preopens, stdio, streams, terminal};

pub fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    // Overrides
    // linker.instance("wasi:io/streams")?.func_wrap(
    //     "read",
    //     |data: StoreContextMut<T>, (id, max_bytes): (u32, u64)| {
    //         if id != STDIN_IDENT {
    //             bail!("unexpected read stream id: {id}");
    //         }

    //         let mut bytes = vec![0u8; max_bytes as usize];

    //         let (bytes_written, stream_ended) = data.ctx_mut().stdin().read(&mut bytes)?;

    //         bytes.truncate(bytes_written as _);

    //         Ok((bytes, stream_ended))
    //     },
    // )?;

    // linker.instance("wasi:io/streams")?.func_wrap(
    //     "write",
    //     |data: StoreContextMut<T>, (id, buffer): (u32, Vec<u8>)| {
    //         let bytes_written = match id {
    //             STDOUT_IDENT => data.ctx_mut().stdout().write(&buffer)?,

    //             STDERR_IDENT => data.ctx_mut().stderr().write(&buffer)?,

    //             id => bail!("unexpected write stream id: {id}"),
    //         };
    //         Ok(bytes_written)
    //     },
    // )?;

    linker.instance("wasi:random/random")?.func_wrap(
        "get-random-bytes",
        |data: StoreContextMut<T>, (len,): (u64,)| {
            let random = data.ctx_mut().random();
            let mut bytes = vec![0u8; len as _];
            random.fill_bytes(&mut bytes);
            Ok(bytes)
        },
    )?;

    linker
        .instance("wasi:random/random")?
        .func_wrap("get-random-u64", |data: StoreContextMut<T>, (): ()| {
            Ok(data.ctx_mut().random().next_u64())
        })?;

    linker.instance("wasi:cli/exit")?.func_wrap(
        "exit",
        |_data: StoreContextMut<T>,
         (_status,): (std::result::Result<(), ()>,)|
         -> anyhow::Result<()> {
            todo!("exit called");
            // Ok(())
        },
    )?;

    clocks::add_to_linker(linker)?;
    stdio::add_to_linker(linker)?;

    environment::add_to_linker(linker)?;
    preopens::add_to_linker(linker)?;
    filesystem::add_to_linker(linker)?;
    terminal::add_to_linker(linker)?;
    streams::add_to_linker(linker)?;

    Ok(())
}
