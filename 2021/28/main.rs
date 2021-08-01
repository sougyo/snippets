fn main() {
  let s = r###"aaaa"bbbb"###;
  let t = b"hello, world";

  println!("{}", s);
  println!("{:?}", &t[1..10]);
}
