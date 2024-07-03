#[macro_export]
macro_rules! define_wasm_import {
  ($name:ident, ($($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
    paste::paste! {
      extern {
        fn [<wasm_ $name>]($($arg_name : u64),*) -> define_wasm_import!(@@return_type $($ret)?);
      }

      unsafe fn $name($($arg_name : $arg_ty),*) $(-> $ret)? {
        let raw = [<wasm_ $name>]($(
          <$arg_ty as WasmIntoAbi>::into_u64(&$arg_name)
        ),*);
        define_wasm_import!(@@return_value raw $($ret)?)
      }
    }
  };
  (@@return_type $ty:ty) => { u64 };
  (@@return_type ) => { () };
  (@@return_value $raw:ident $ty:ty) => { <$ty as WasmFromAbi>::from_u64_owned($raw) };
  (@@return_value $raw:ident ) => { () };
}
