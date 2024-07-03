use crate::buffer::WasmBuffer;

pub trait WasmIntoAbi {
  unsafe fn into_u64(&self) -> u64;
  unsafe fn into_u64_unown(&mut self) -> u64;
}

impl WasmIntoAbi for WasmBuffer {
  unsafe fn into_u64(&self) -> u64 {
    let wasm_ptr = std::mem::transmute::<[u32; 2], u64>([self.ptr as u32, self.len as u32]);
    return wasm_ptr;
  }

  unsafe fn into_u64_unown(&mut self) -> u64 {
    let wasm_ptr = self.into_u64();
    self.owned = false;
    return wasm_ptr;
  }
}

impl WasmIntoAbi for String {
  unsafe fn into_u64(&self) -> u64 {
    let buffer = WasmBuffer::from_str(self.as_str());
    let result = buffer.into_u64();
    return result;
  }

  unsafe fn into_u64_unown(&mut self) -> u64 {
    let mut buffer = WasmBuffer::new_from_str(self.as_str());
    buffer.owned = false;
    let result = buffer.into_u64();
    return result;
  }
}

impl WasmIntoAbi for u64 {
  unsafe fn into_u64(&self) -> u64 {
    return *self;
  }

  unsafe fn into_u64_unown(&mut self) -> u64 {
    return *self;
  }
}

impl WasmIntoAbi for f64 {
  unsafe fn into_u64(&self) -> u64 {
    return *self as u64;
  }

  unsafe fn into_u64_unown(&mut self) -> u64 {
    return *self as u64;
  }
}
