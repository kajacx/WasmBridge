use super::*;
use crate::conversions::ToJsValue;
use crate::Result;
use wasm_bindgen::JsValue;

macro_rules! lower_primitive {
    ($ty: ty) => {
        impl Lower for $ty {
            fn to_js_args<M: WriteableMemory>(
                &self,
                args: &mut Vec<JsValue>,
                _memory: &M,
            ) -> Result<()> {
                args.push(self.to_js_value());
                Ok(())
            }

            fn to_js_return<M: WriteableMemory>(&self, _memory: &M) -> Result<JsValue> {
                Ok(self.to_js_value())
            }

            fn write_to<M: WriteableMemory>(
                &self,
                buffer: &mut ByteBuffer,
                _memory: &M,
            ) -> Result<()> {
                buffer.write(&self.to_le_bytes());
                Ok(())
            }
        }
    };
}

lower_primitive!(u8);
lower_primitive!(u16);
lower_primitive!(u32);
lower_primitive!(u64);

lower_primitive!(i8);
lower_primitive!(i16);
lower_primitive!(i32);
lower_primitive!(i64);

lower_primitive!(f32);
lower_primitive!(f64);

impl Lower for bool {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()> {
        (*self as u8).to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        (*self as u8).to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        (*self as u8).write_to(buffer, memory)
    }
}

impl Lower for char {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()> {
        (*self as u32).to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        (*self as u32).to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        (*self as u32).write_to(buffer, memory)
    }
}

impl<T: Lower> Lower for &[T] {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()> {
        let addr = write_vec_data(self, memory)? as u32;
        let len = self.len() as u32;

        // First address, then element count
        args.push(addr.into());
        args.push(len.into());

        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        let mut buffer = memory.allocate(T::alignment(), T::flat_byte_size() * self.len())?;
        self.write_to(&mut buffer, memory)?;

        let addr = memory.flush(buffer) as u32;
        Ok(addr.to_js_value())
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        let addr = write_vec_data(self, memory)? as u32;
        let len = self.len() as u32;

        addr.write_to(buffer, memory)?;
        len.write_to(buffer, memory)?;

        Ok(())
    }
}

impl<T: Lower> Lower for Vec<T> {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()> {
        self.as_slice().to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        self.as_slice().to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.as_slice().write_to(buffer, memory)
    }
}

impl Lower for &str {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()> {
        self.as_bytes().to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        self.as_bytes().to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.as_bytes().write_to(buffer, memory)
    }
}

impl Lower for String {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()> {
        self.as_bytes().to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        self.as_bytes().to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.as_bytes().write_to(buffer, memory)
    }
}

// Writes the data to the memory, returning the starting address of the data
fn write_vec_data<T: Lower, M: WriteableMemory>(data: &[T], memory: &M) -> Result<usize> {
    // Allocate space for all the elements
    let mut buffer = memory.allocate(T::alignment(), T::flat_byte_size() * data.len())?;

    // Then write the elements to the slice buffer
    for elem in data {
        elem.write_to(&mut buffer, memory)?;
    }

    // Then actually write the slice buffer to memory and return the address
    Ok(memory.flush(buffer))
}

impl Lower for () {
    fn to_js_args<M: WriteableMemory>(&self, _args: &mut Vec<JsValue>, _memory: &M) -> Result<()> {
        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, _memory: &M) -> Result<JsValue> {
        Ok(JsValue::UNDEFINED)
    }

    fn write_to<M: WriteableMemory>(&self, _buffer: &mut ByteBuffer, _memory: &M) -> Result<()> {
        Ok(())
    }
}

impl<T: Lower> Lower for (T,) {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()> {
        self.0.to_js_args(args, memory)
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        self.0.to_js_return(memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.0.write_to(buffer, memory)
    }
}

impl<T: Lower, U: Lower> Lower for (T, U) {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()> {
        self.0.to_js_args(args, memory)?;
        self.1.to_js_args(args, memory)?;
        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        let mut buffer = memory.allocate(Self::alignment(), Self::flat_byte_size())?;
        self.write_to(&mut buffer, memory)?;

        let addr = memory.flush(buffer) as u32;
        Ok(addr.to_js_value())
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        // CAREFUL!!!
        // `write_to` needs to fill the entire byte size of the pair,
        // or there would be unfilled "gaps" and the data would get shifted.
        let layout = Self::layout();

        self.0.write_to(buffer, memory)?;
        buffer.skip(layout[2] - layout[1]);

        self.1.write_to(buffer, memory)?;
        buffer.skip(layout[4] - layout[2]);

        Ok(())
    }
}

impl<T: Lower, U: Lower, V: Lower> Lower for (T, U, V) {
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()> {
        self.0.to_js_args(args, memory)?;
        self.1.to_js_args(args, memory)?;
        self.2.to_js_args(args, memory)?;
        Ok(())
    }

    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        let mut buffer = memory.allocate(Self::alignment(), Self::flat_byte_size())?;
        self.write_to(&mut buffer, memory)?;

        let addr = memory.flush(buffer) as u32;
        Ok(addr.to_js_value())
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        // CAREFUL!!!
        // `write_to` needs to fill the entire byte size of the tuple,
        // or there would be unfilled "gaps" and the data would get shifted.
        let layout = Self::layout();

        self.0.write_to(buffer, memory)?;
        buffer.skip(layout[2] - layout[1]);

        self.1.write_to(buffer, memory)?;
        buffer.skip(layout[4] - layout[2]);

        self.2.write_to(buffer, memory)?;
        buffer.skip(layout[6] - layout[4]);

        Ok(())
    }
}
