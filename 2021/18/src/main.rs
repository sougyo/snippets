async fn func() -> i32 {
    100
}

#[tokio::main]
async fn main() {
    println!("{}", func().await);
}
