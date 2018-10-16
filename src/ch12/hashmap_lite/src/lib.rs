#[macro_export]
macro_rules! hashmap {
    (@unit $($x:tt)*) => (());
    (@count $($rest:expr),*) => 
        (<[()]>::len(&[$(hashmap!(@unit $rest)),*]));
    ($($key:expr => $value:expr),* $(,)*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = 
                ::std::collections::HashMap::with_capacity(_cap);
           $(
               _map.insert($key, $value);
           )*
           _map
       }
   };
}

//  测试$crate
pub fn incr(x: u32) -> u32 { 
    x+1
}

#[macro_export]
macro_rules! inc {
    ($x:expr) => ( $crate::incr($x) )
}