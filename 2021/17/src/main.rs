use async_std::task;

async fn func() -> i32 {
    println!("func called");
    100
}

fn main() {
    let p = async {
        let x = func().await;
        println!("{}", x);
    };
    println!("before block_on");
    task::block_on(p);
    println!("after block_on");
}
