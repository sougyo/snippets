fn main() {
  let x: Option<i32> = Some(2);
  let y: Option<i32> = Some(3);
  let n: Option<i32> = None;
  
  println!("{:?}", x.zip(y).map(|a| a.0 + a.1)); // Some(5)
  println!("{:?}", x.zip(n).map(|a| a.0 + a.1)); // None
  println!("{:?}", n.zip(y).map(|a| a.0 + a.1)); // None
  println!("{:?}", n.zip(n).map(|a| a.0 + a.1)); // None
}
