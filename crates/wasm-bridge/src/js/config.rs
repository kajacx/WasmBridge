#[derive(Debug, Default)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }

    #[cfg(feature = "component-model")]
    pub fn wasm_component_model(&mut self, _: bool) -> &mut Self {
        self
    }
}
