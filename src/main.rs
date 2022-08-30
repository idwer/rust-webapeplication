use std::env;

mod ape;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        panic!("You need to pass height and wingspan!");
    }

    let height: u32 = args[1].parse::<u32>().unwrap();
    let wingspan: u32 = args[2].parse::<u32>().unwrap();

    if height < 1 || wingspan < 1 {
        panic!("height or wingspan found to be unreasonably short (debug: height {}, wingspan {})", height, wingspan);
    }

    let ape_index = ape::ape_index(height, wingspan);

    println!("ape index: {ape_index}");
}