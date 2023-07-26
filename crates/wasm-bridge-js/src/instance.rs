use crate::{helpers::map_js_error, *};
use anyhow::bail;
use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::JsValue;

pub struct Instance {
    instance: WebAssembly::Instance,
    exports: JsValue,
    _closures: Vec<DropHandler>,
}

impl Instance {
    pub fn new(_store: impl AsContextMut, module: &Module, _: impl AsRef<[()]>) -> Result<Self> {
        let imports = Object::new();
        Self::new_with_imports(module, &imports, vec![])
    }

    pub(crate) fn new_with_imports(
        module: &Module,
        imports: &Object,
        closures: Vec<DropHandler>,
    ) -> Result<Self> {
        let instance = WebAssembly::Instance::new(&module.module, imports)
            .map_err(map_js_error("Instantiate WebAssembly module"))?;
        let exports = Reflect::get(instance.as_ref(), &"exports".into())
            .map_err(map_js_error("Get instance's exports"))?;
        Ok(Self {
            instance,
            exports,
            _closures: closures,
        })
    }

    pub fn get_typed_func<Params: ToJsValue, Results: FromJsValue>(
        &self,
        _store: impl AsContextMut,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>, Error> {
        let function = Reflect::get(&self.exports, &name.into())
            .map_err(map_js_error("Get exported fn with reflect"))?;

        if !function.is_function() {
            bail!("Exported function with name '{name}' not found");
        }

        let function: Function = function.into();

        if function.length() != Params::number_of_args() {
            bail!(
                "Exported function {name} should have {} arguments, but it has {} instead.",
                Params::number_of_args(),
                function.length(),
            );
        }

        Ok(TypedFunc::new(self.instance.clone(), function))
    }

    pub fn get_func(&self, _store: impl AsContextMut, name: &str) -> Option<Func> {
        let function = Reflect::get(&self.exports, &name.into()).ok()?;

        if !function.is_function() {
            return None;
        }

        let function: Function = function.into();

        Some(Func::new(self.instance.clone(), function))
    }

    pub fn get_memory<T>(&self, _: &mut Store<T>, id: &str) -> Option<Memory> {
        let memory = Reflect::get(&self.exports, &id.into()).ok()?.into();
        Some(Memory { memory })
    }
}
