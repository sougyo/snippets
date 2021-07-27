#[derive(Debug)]
struct A {
	a: i64
}

fn main() {
	let x = [ A { a: 1 }, A { a: 2 }, A { a: 3 } ];          // [A; 3]

	let z1 =        x.iter().map(|n| n).collect::<Vec<_>>(); // [A; 3] -> Vec<&A>
	let z2 =    x[..].iter().map(|n| n).collect::<Vec<_>>(); // [A]    -> Vec<&A>
	let z3 = (&x[..]).iter().map(|n| n).collect::<Vec<_>>(); // &[A]   -> Vec<&A>

	println!("{:?}", x);
	println!("{:?}", z1);
	println!("{:?}", z2);
	println!("{:?}", z3);
}
