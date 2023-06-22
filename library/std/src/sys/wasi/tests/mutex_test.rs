use std::sync::{Arc, Mutex};

fn main() {
    let mut join = vec![];
    let threads_num = 4;
    let iterations = 200;
    let mutex = Arc::new(Mutex::new(0));
    for _ in 0..threads_num {
        let c_mutex = Arc::clone(&mutex);
        join.push(std::thread::spawn(move || {
            for _ in 0..iterations {
                let mut num = c_mutex.lock().unwrap();
                *num += 1;
            }
        }));
    }
    for j in join {
        j.join().unwrap();
    }
    assert_eq!(*mutex.lock().unwrap(), threads_num * iterations);
}
