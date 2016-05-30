mod thread;
mod thread_pool;

use thread::{State, Thread, ThreadId};
use thread_pool::{ThreadPool};



/// Run 1
/// - thread1 send
/// - thread2 receive
/// - thread2 reply
/// - both ready
fn run_1() {
    println!("Run 1");
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

fn main() {
    run_1();

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
    println!("\nseparator\n");

    let thread3 = thread_pool.create();
    let thread4 = thread_pool.create();
    thread_pool.print();
    thread_pool.receive(thread4, thread3);
    thread_pool.print();
    thread_pool.send(thread3, thread4, "Hi".to_string());
    thread_pool.print();
}

