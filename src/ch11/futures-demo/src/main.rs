#![feature(arbitrary_self_types, futures_api, async_await, await_macro, pin)]

// use futures::{
//     executor::{ThreadPool,LocalPool},
//     task::{SpawnExt, LocalSpawnExt},
// };
use futures::{
    executor::ThreadPool,
    task::SpawnExt,
};
use std::future::{Future};
use std::pin::Pin;
use std::task::*;
// use std::thread;
// use std::time::Duration;

pub struct AlmostReady {
    ready: bool,
    value: i32,
}

pub fn almost_ready(value: i32) -> AlmostReady {
    AlmostReady { ready: false, value }
}

impl Future for AlmostReady {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, lw: &LocalWaker)
        -> Poll<Self::Output>
    {
        if self.ready {
            Poll::Ready(self.value + 1)
        } else {
            unsafe { Pin::get_mut_unchecked(self).ready = true ;}
            lw.wake();
            Poll::Pending
        }
    }
}

fn main() {
    let mut executor = ThreadPool::new().unwrap();
    let future = async {
        println!("howdy!");
        let x = await!(almost_ready(5));
        println!("done: {:?}", x);
    };
    // executor.spawn(future).unwrap();
    // let five_seconds = Duration::new(1, 0);
    // thread::sleep(five_seconds);

    executor.run(future);

    // // LocalPool run
    // let mut executor = LocalPool::new();
    // // let mut spawner = executor.spawner();
    // let future = async {
    //     println!("howdy!");
    //     let x = await!(almost_ready(5));
    //     println!("done: {:?}", x);
    // };

    // // spawner.spawn_local(future).unwrap();
    // executor.run_until(future);
}

