fn main() {
   let x = "  aa bb   cc  "
     .split_whitespace()
     .collect::<Vec<_>>();

   println!("{:?}", x); // ["aa", "bb", "cc"]
}
