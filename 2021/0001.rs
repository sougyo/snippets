fn func1<F: Fn(i32, i32) -> i32>(f: F) -> i32 {
  f(1, 2)
}

fn func2(x: i32, y: i32) -> i32 {
  x + y
}

fn main() {
  println!("{}", func1(func2));
}
