pub fn ape_index(h: u32, w: u32) -> f32 {
    w as f32 / h as f32
}

#[cfg(test)]
mod tests_ape_cli {
    use super::*;

    #[test]
    fn test_ape_index() {
        assert_eq!(ape_index(187, 193), 1.0320855);
    }
}
