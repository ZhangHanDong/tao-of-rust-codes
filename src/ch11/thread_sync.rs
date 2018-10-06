/// # 线程同步
///
/// Basic usage: 线程之间传递可变字符
/// 
/// 该代码存在数据竞争
/// 
/// ```rust
/// use std::thread;
/// fn main() {
///     let mut s = "Hello".to_string();
///     for _ in 0..3 {
///         thread::spawn(move || {
///             s.push_str(" Rust!");
///         });
///     }
/// }
/// ```
///
/// Basic usage: 使用Rc<T>共享所有权
/// 
/// Rc<T>不能用于多线程共享
/// 
/// ```rust
/// use std::thread;
/// use std::rc::Rc;
/// fn main() {
///     let mut s = Rc::new("Hello".to_string());
///     for _ in 0..3 {
///         let mut s_clone = s.clone();
///         thread::spawn(move || {
///             s_clone.push_str(" hello");
///         });
///    }
/// }
/// ```
///
/// Basic usage: 使用Arc<T>共享所有权
/// 
/// Arc<T>内部不可变，所以报错
/// 
/// ```rust
/// use std::thread;
/// use std::sync::Arc;
/// fn main() {
///     let s = Arc::new("Hello".to_string());
///     for _ in 0..3 {
///         let s_clone = s.clone();
///         thread::spawn(move || {
///             s_clone.push_str(" world!");
///         });
///    }
/// }
/// ```
///
/// Basic usage: 使用RefCell支持内部可变
/// 
/// RefCell<String>没有实现Sync，报错
/// 
/// ```rust
/// use std::thread;
/// use std::sync::Arc;
/// use std::cell::RefCell;
/// fn main() {
///     let s = Arc::new(RefCell::new("Hello".to_string()));
///     for _ in 0..3 {
///         let s_clone = s.clone();
///         thread::spawn(move || {
///             let s_clone = s_clone.borrow_mut();
///            s_clone.push_str(" world!");
///        });
///    }
/// }
/// ```
/// 
/// Basic usage: 使用Metux
/// 
/// 
/// ```rust
/// use std::thread;
/// use std::sync::{Arc, Mutex};
/// fn main() {
///     let s = Arc::new(Mutex::new("Hello".to_string()));
///     let mut v = vec![];
///     for _ in 0..3 {
///         let s_clone = s.clone();
///         let child = thread::spawn(move || {
///             let mut s_clone = s_clone.lock().unwrap();
///             s_clone.push_str(" world!");
///        });
///        v.push(child);
///    }
///    for child in v {
///         child.join().unwrap();
///    }
/// }
/// ```
/// 
/// Basic usage: 跨线程恐慌“中毒”示例
/// 
/// 线程在获得锁之后发生恐慌
/// 
/// ```rust
/// use std::sync::{Arc, Mutex};
/// use std::thread;
/// fn main() {
///     let mutex = Arc::new(Mutex::new(1));
///     let c_mutex = mutex.clone();
///     let _ = thread::spawn(move || {
///         let mut data = c_mutex.lock().unwrap();
///         *data = 2;
///         panic!("oh no");
///    }).join();
///    assert_eq!(mutex.is_poisoned(), true);
///    match mutex.lock() {
///        Ok(_) => unreachable!(),
///        Err(p_err) => {
///            let data = p_err.get_ref();
///            println!("recovered: {}", data);
///        }
///    };
/// }
/// ```
/// 
/// 
/// Basic usage: 投掷硬币示例
/// 
/// 不会死锁
/// 
/// ```rust
/// extern crate rand;
/// use std::thread;
/// use std::sync::{Arc, Mutex};
/// fn main() {
///     let total_flips = Arc::new(Mutex::new(0));
///     let completed = Arc::new(Mutex::new(0));
///     let runs = 8;
///     let target_flips = 10;
///     for _ in 0..runs {
///        let total_flips = total_flips.clone();
///        let completed = completed.clone();
///        thread::spawn(move || {
///            flip_simulate(target_flips, total_flips);
///            let mut completed = completed.lock().unwrap();
///            *completed += 1;
///        });
///    }
///    loop {
///        let completed = completed.lock().unwrap();
///        if *completed == runs {
///            let total_flips = total_flips.lock().unwrap();
///            println!("Final average: {}", *total_flips / *completed);
///            break;
///        }
///    }
/// }
/// 
/// fn flip_simulate(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
///     let mut continue_positive = 0;
///     let mut iter_counts = 0;
///     while continue_positive <= target_flips {
///         iter_counts += 1;
///         let pro_or_con = rand::random();
///         if pro_or_con {
///            continue_positive += 1;
///         } else {
///            continue_positive = 0;
///        }
///    }
///    println!("iter_counts: {}", iter_counts);
///    let mut total_flips = total_flips.lock().unwrap();
///    *total_flips += iter_counts;
/// }
/// ```
/// 
/// Basic usage: 投掷硬币示例
/// 
/// 死锁
/// 
/// ```rust
/// extern crate rand;
/// use std::thread;
/// use std::sync::{Arc, Mutex};
/// fn main() {
///     let total_flips = Arc::new(Mutex::new(0));
///     let completed = Arc::new(Mutex::new(0));
///     let runs = 8;
///     let target_flips = 10;
///     for _ in 0..runs {
///        let total_flips = total_flips.clone();
///        let completed = completed.clone();
///        thread::spawn(move || {
///            flip_simulate(target_flips, total_flips);
///            let mut completed = completed.lock().unwrap();
///            *completed += 1;
///        });
///    }
///    loop {
///        let completed = completed.lock().unwrap();
///        while *completed < runs {}
///        let total_flips = total_flips.lock().unwrap();
///        println!("Final average: {}", *total_flips / *completed);
///    }
/// }
/// 
/// fn flip_simulate(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
///     let mut continue_positive = 0;
///     let mut iter_counts = 0;
///     while continue_positive <= target_flips {
///         iter_counts += 1;
///         let pro_or_con = rand::random();
///         if pro_or_con {
///            continue_positive += 1;
///         } else {
///            continue_positive = 0;
///        }
///    }
///    println!("iter_counts: {}", iter_counts);
///    let mut total_flips = total_flips.lock().unwrap();
///    *total_flips += iter_counts;
/// }
/// ```
/// 
/// Basic usage: 读写锁示例
/// 
/// 
/// ```rust
/// use std::sync::RwLock;
/// fn main() {
///     let lock = RwLock::new(5);
///     {
///         let r1 = lock.read().unwrap();
///         let r2 = lock.read().unwrap();
///         assert_eq!(*r1, 5);
///         assert_eq!(*r2, 5);
///     } 
///    {
///        let mut w = lock.write().unwrap();
///        *w += 1;
///        assert_eq!(*w, 6);
///    }
/// }
/// ```
/// 
/// Basic usage: 屏障示例
/// 
/// 
/// ```rust
/// use std::sync::{Arc, Barrier};
/// use std::thread;
/// fn main() {
///     let mut handles = Vec::with_capacity(5);
///     let barrier = Arc::new(Barrier::new(5));
///     for _ in 0..5 {
///         let c = barrier.clone();
///         handles.push(thread::spawn(move|| {
///             println!("before wait");
///            c.wait();
///            println!("after wait");
///        }));
///    }
///    for handle in handles {
///        handle.join().unwrap();
///    }
/// }
/// ```
/// 
/// Basic usage: 条件变量
/// 
/// 
/// ```rust
/// use std::sync::{Arc, Condvar, Mutex};
/// use std::thread;
/// fn main() {
///     let pair = Arc::new((Mutex::new(false), Condvar::new()));
///     let pair_clone = pair.clone();
///     thread::spawn(move || {
///         let &(ref lock, ref cvar) = &*pair_clone;
///         let mut started = lock.lock().unwrap();
///         *started = true;
///        cvar.notify_one();
///    });
///    let &(ref lock, ref cvar) = &*pair;
///    let mut started = lock.lock().unwrap();
///    while !*started {
///        println!("{}", started); // false
///        started = cvar.wait(started).unwrap();
///        println!("{}", started); // true
///    }
/// }
/// ```
pub fn thread_safe(){
    unimplemented!();
}