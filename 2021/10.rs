// http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch16-02-message-passing.html

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

#[derive(Debug)] 
struct MyStruct {
    name: String
}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            MyStruct { name: String::from("hi") },
            MyStruct { name: String::from("test") },
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let child = thread::spawn(move || {
        for received in rx {
            println!("Got: {:?}", received);
        }
    });

    let _ = child.join();
}
