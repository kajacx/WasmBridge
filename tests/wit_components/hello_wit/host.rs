use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

mod wasmtime {
    pub use wasm_bridge::*;
}

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "test-world",
});

struct HostData {
    number: i32,
}

impl TestWorldImports for HostData {
    fn add_b(&mut self, text: String) -> Result<String> {
        Ok(text + "b")
    }

    fn add_numbers_import(&mut self, a: i32, b: i32) -> Result<i32> {
        Ok(a + b)
    }

    fn increment(&mut self) -> Result<()> {
        self.number += 1;
        Ok(())
    }

    fn add_all(
        &mut self,
        a: i32,
        b: i64,
        c: u32,
        d: u64,
        e: f32,
        f: f64,
        g: String,
    ) -> Result<f64> {
        Ok(a as f64 + b as f64 + c as f64 + d as f64 + e as f64 + f + g.parse::<f64>().unwrap())
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostData { number: 0 });

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    TestWorld::add_to_linker(&mut linker, |data| data)?;

    let (instance, _do_not_drop) = TestWorld::instantiate(&mut store, &component, &linker)?;

    let result = instance.call_add_hello(&mut store, "world")?;
    assert_eq!(result, "Hello world");

    let result = instance.call_add_abc(&mut store, "Hello ")?;
    assert_eq!(result, "Hello abc");

    let result = instance.call_add_numbers(&mut store, 5, 6)?;
    assert_eq!(result, 11);

    instance.call_increment_twice(&mut store)?;
    assert_eq!(store.data().number, 2);

    let result = instance.call_add_all_and_one(
        &mut store, 10i32, 20i64, 30u32, 40u64, 50.25f32, 60.25f64, "70",
    )?;
    assert_eq!(
        result,
        10.0 + 20.0 + 30.0 + 40.0 + 50.25 + 60.25 + 70.0 + 1.0
    );

    Ok(())
}
