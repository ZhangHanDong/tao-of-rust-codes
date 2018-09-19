use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;
pub const NF: &'static str = "not found";

lazy_static! {
    pub static ref MAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m
   };
   pub static ref MAP_MUT: RwLock<HashMap<u32, &'static str>> =
   {
       let mut m = HashMap::new();
       m.insert(0, "bar");
       RwLock::new(m)
  };
}
