use std::sync::atomic::AtomicU64;
use std::sync::Arc;

fn main() {
    let mut join = vec![];
    let threads_num = 4;
    let iterations = 200;
    let atomic = Arc::new(AtomicU64::new(0));
    for _ in 0..threads_num {
        let atomic2 = atomic.clone();
        join.push(std::thread::spawn(move || {
            for _ in 0..iterations {
                atomic2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }   
        }));
    }
    for j in join {
        j.join().unwrap();
    }
    assert_eq!(atomic.load(std::sync::atomic::Ordering::SeqCst), threads_num * iterations);
}