
use itertools::Itertools;
use crypto::{
    digest::Digest,
    sha2::Sha256,
};
use std::{
    thread,
    sync::{mpsc, Arc},
    sync::atomic::{AtomicBool, Ordering},
};

const BASE: usize = 42;
const THREADS: usize = 8;
static DIFFICULTY: &'static str = "00000";
struct Solution(usize, String);

fn verify(number: usize) -> Option<Solution> {
    let mut hasher = Sha256::new();
    hasher.input_str(&(number * BASE).to_string());
    let hash: String = hasher.result_str();
    if hash.starts_with(DIFFICULTY) { 
        Some(Solution(number, hash))
    } else { None }
}

fn find(
    start_at: usize, 
    sender: mpsc::Sender<Solution>, 
    is_solution_found: Arc<AtomicBool>
) {
    for number in (start_at..).step(THREADS) {
        if is_solution_found.load(Ordering::Relaxed) { return; }
        if let Some(solution) = verify(number) {
            is_solution_found.store(true, Ordering::Relaxed);
            sender.send(solution).unwrap();
            return;
        }
   }
}

fn main() {
    println!("PoW : Find a number,
        SHA256(the number  * {}) == \"{}......\" ", BASE, DIFFICULTY);
    println!("Started {} threads", THREADS);
    println!("Please wait...  ");
    let is_solution_found = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = mpsc::channel();
    for i in 0..THREADS {
        let sender_n = sender.clone();
        let is_solution_found = is_solution_found.clone();
        thread::spawn(move || {
            find(i, sender_n, is_solution_found);
       });
   }
   match receiver.recv() {
       Ok(Solution(i, hash)) => {
           println!("Found the solution: ");
           println!("The number is: {}, 
                       and hash result is : {}.", i, hash);
       },
       Err(_) => panic!("Worker threads disconnected!"),
   }
}
