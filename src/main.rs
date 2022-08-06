use std::env;

fn ape_index(h: u32, w: u32) -> f32 {
    w as f32 / h as f32
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let height: u32 = args[1].parse::<u32>().unwrap();
    let wingspan: u32 = args[2].parse::<u32>().unwrap();

    if height < 1 || wingspan < 1 {
        panic!("height or wingspan found to be unreasonably short (debug: height {}, wingspan {})", height, wingspan);
    }

    println!("ape index: {}", ape_index(height, wingspan));
}

#[cfg(test)]
mod tests_ape_cli {
    use super::*;

    #[test]
    fn test_ape_index() {
        assert_eq!(ape_index(187, 193), 1.0320855);
    }
}
