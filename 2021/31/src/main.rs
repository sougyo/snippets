use std::io::Read;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let vec_args = std::env::args().collect::<Vec<String>>();

    if vec_args.len() != 2 {
        println!("Usage: ./str <filename>");
        return Ok(());
    }

    let mut f = File::open(&vec_args[1])?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    Ok(())
}
