use js_sys::Function;
use wasm_bindgen::JsValue;

use wasm_bridge::Result;
use wasm_bridge_js::helpers::map_js_error;

use super::StreamStatus;

pub trait OutputStream: Send {
    fn as_any(&self) -> &dyn std::any::Any;

    fn writable(&self) -> Result<()>;

    fn write(&mut self, buf: &[u8]) -> Result<(usize, StreamStatus)>;
}

struct VoidingStream;

impl OutputStream for VoidingStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn writable(&self) -> Result<()> {
        Ok(())
    }

    /// Non blocking write
    fn write(&mut self, buf: &[u8]) -> Result<(usize, StreamStatus)> {
        Ok((buf.len(), StreamStatus::Open))
    }
}

pub(crate) fn voiding_stream() -> impl OutputStream {
    VoidingStream
}

struct InheritStream(String);

impl OutputStream for InheritStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn writable(&self) -> Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<(usize, StreamStatus)> {
        let text = String::from_utf8_lossy(buf);

        // Do not store the js Function, it makes the stream not Send
        let function: Function = js_sys::eval(&self.0)
            .map_err(map_js_error("Eval inherit stream function"))?
            .into();
        debug_assert!(function.is_function());

        function
            .call1(&JsValue::UNDEFINED, &text.as_ref().into())
            .map_err(map_js_error("Call output stream function"))?;

        Ok((buf.len() as _, StreamStatus::Open))
    }
}

pub(crate) fn console_log_stream() -> impl OutputStream {
    InheritStream("console.log".into())
}

pub(crate) fn console_error_stream() -> impl OutputStream {
    InheritStream("console.error".into())
}
