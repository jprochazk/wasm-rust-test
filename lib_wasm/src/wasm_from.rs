use crate::buffer::WasmBuffer;

pub trait WasmFromAbi {
  unsafe fn from_u64(v: u64) -> Self;
  unsafe fn from_u64_owned(v: u64) -> Self;
}

impl WasmFromAbi for WasmBuffer {
  unsafe fn from_u64(v: u64) -> Self {
    let [ptr, len] = std::mem::transmute::<u64, [u32; 2]>(v);
    return Self {
      ptr: ptr as *mut u8,
      len: len as usize,
      owned: false,
    };
  }

  unsafe fn from_u64_owned(v: u64) -> Self {
    let mut result = Self::from_u64(v);
    result.owned = true;
    return result;
  }
}

impl WasmFromAbi for String {
  unsafe fn from_u64(v: u64) -> Self {
    let buffer = WasmBuffer::from_u64(v);
    let result = buffer.to_str().to_string();
    return result;
  }

  unsafe fn from_u64_owned(v: u64) -> Self {
    let buffer = WasmBuffer::from_u64_owned(v);
    let result = buffer.to_str().to_string();
    return result;
  }
}

impl WasmFromAbi for u64 {
  unsafe fn from_u64(v: u64) -> Self {
    return v;
  }

  unsafe fn from_u64_owned(v: u64) -> Self {
    return v;
  }
}

impl WasmFromAbi for f64 {
  unsafe fn from_u64(v: u64) -> Self {
    return v as f64;
  }

  unsafe fn from_u64_owned(v: u64) -> Self {
    return v as f64;
  }
}
