fn main() {
  let mut v = vec![(1,2),(4,5),(2,1)];
  v.sort_by( |a, b| (&a).0.partial_cmp(&b.0).unwrap() );
  for e in v {
    println!("{:?}", e);
  }
}

