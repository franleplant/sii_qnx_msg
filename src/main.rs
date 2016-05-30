extern crate ansi_term;

mod thread;
mod thread_pool;

//use thread::{State, Thread, ThreadId};
use thread_pool::{ThreadPool};



/// Run 1
/// - thread1 send
/// - thread2 receive
/// - thread2 reply
/// - both ready
fn run_1() {
    println!("Run 1\n--------\n");
    let mut thread_pool = ThreadPool::new();
    let thread1_id = thread_pool.create();
    let thread2_id = thread_pool.create();
    thread_pool.print();
    thread_pool.send(thread1_id, thread2_id, "Hi".to_string());
    thread_pool.print();
    thread_pool.receive(thread2_id, thread1_id);
    thread_pool.print();
    thread_pool.reply(thread2_id, thread1_id, "How are you".to_string());
    thread_pool.print();
}

/// Run 2
/// - thread2 receive
/// - thread1 send
/// - thread2 reply
/// - both ready
fn run_2() {
    println!("Run 2\n--------\n");
    let mut thread_pool = ThreadPool::new();
    let thread1_id = thread_pool.create();
    let thread2_id = thread_pool.create();
    thread_pool.print();
    thread_pool.receive(thread2_id, thread1_id);
    thread_pool.print();
    thread_pool.send(thread1_id, thread2_id, "Hi".to_string());
    thread_pool.print();
    thread_pool.reply(thread2_id, thread1_id, "How are you".to_string());
    thread_pool.print();
}


/// run 3
/// - Thread 1 Receive Blocked 2
/// - Thread 2 Send Blocked 2
/// - Both become unlocked
fn run_3() {
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;
    println!("Run 3\n--------\n");

    let (tx, rx) = mpsc::channel();

    {
        let tx = tx.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            println!("Thread 2 Send 1");
            tx.send("How are you").unwrap();
            println!("Thread 2 Unblocked");
        });
    }

    println!("Thread 1 Receive 2");
    let msg = rx.recv().unwrap();
    println!("Thread 1 Unblocked");
    println!("{}", msg);
}

fn main() {
    run_1();
    run_2();
    run_3();
}

