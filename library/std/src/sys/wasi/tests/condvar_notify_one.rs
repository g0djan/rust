use std::sync::{Arc, Condvar, Mutex};

fn main() {
    let m = Arc::new(Mutex::new(()));
    let m2 = m.clone();
    let c = Arc::new(Condvar::new());
    let c2 = c.clone();

    let g = m.lock().unwrap();
    let _t = std::thread::spawn(move || {
        let _g = m2.lock().unwrap();
        c2.notify_one();
    });
    let g = c.wait(g).unwrap();
    drop(g);
}
