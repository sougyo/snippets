fn main() {
    let result = (1..=3).into_iter()
        .map(|x| (4..=6).into_iter().map(move |y| (x, y)))
        .flatten().collect::<Vec<_>>();
    println!("{:?}", result);
}
