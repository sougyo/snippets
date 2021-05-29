fn main() {
  let v = vec![1,2,3,4,5];
  let t = v[1..]
    .iter()
    .zip(v.iter())
    .filter(|x| { *x.1 > 2 });

  for e in t {
    println!("{:?}", e);
  }
}
