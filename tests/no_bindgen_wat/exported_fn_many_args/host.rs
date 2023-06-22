use std::error::Error;
use wasm_bridge::*;

pub fn run_test(wat: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), wat).unwrap();

    let instance = Instance::new(&mut store, &module, &[])?;

    let add_ten_all =
        instance.get_typed_func::<(i32, i64), (i32, i64)>(&mut store, "add_ten_all")?;

    let results = add_ten_all.call(&mut store, (10, 20))?;
    assert_eq!(results, (20, 30));

    Ok(())
}