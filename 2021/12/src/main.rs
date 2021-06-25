use std::{thread, time};
fn main() {
    let ten_millis = time::Duration::from_secs(30);
    let mut v = Vec::new();

    for _ in 0..(1720*1024*1024){
        v.push(3);
    }

    thread::sleep(ten_millis);
}
