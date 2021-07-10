use std::thread;
use std::time::Instant;
use std::sync::mpsc::channel;

const LOOPN: i64 = 1000000;

enum Label {
  Ping,
  Pong,
  Finish
}

use Label::*;

fn main() {
  let (t1to2, r2from1) = channel();
  let (t2to1, r1from2) = channel();
  let (t1to0, r0from1) = channel();

  let start = Instant::now();

  let t1 = thread::spawn(move || {
    let mut cnt = 0;

    t1to2.send(Ping).unwrap();
    loop { 
      match r1from2.recv().unwrap() {
        Pong => {
          cnt += 1;
          if cnt >= LOOPN {
            break;
          } 
          t1to2.send(Ping).unwrap();
        },
        _ => panic!("unexpected")
      }
    } 
    t1to0.send(cnt).unwrap();
    t1to2.send(Finish).unwrap();
  });

  let t2 = thread::spawn(move || {
    loop {
      match r2from1.recv().unwrap() {
        Ping   => t2to1.send(Pong).unwrap(),
        Finish => break,
        _      => panic!("unexpected")
      }
    }
  });

  let result = r0from1.recv().unwrap();
  let end = start.elapsed();

  let _ = t1.join();
  let _ = t2.join();

  println!("{:?}", 1000.0 * (result as f64) / (end.as_millis() as f64));
}
