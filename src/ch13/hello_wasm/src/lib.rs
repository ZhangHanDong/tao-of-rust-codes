#[link(wasm_import_module = "env")]
extern "C" {
  pub fn logit();
  pub fn hello(ptr: *const u8, len: u32);
}
#[no_mangle]
pub extern "C" fn add_one(x: i32) {
    unsafe {
        logit();
        let msg = format!("Hello world: {}", x + 1);
        hello(msg.as_ptr(), msg.len() as u32);
    }
}
