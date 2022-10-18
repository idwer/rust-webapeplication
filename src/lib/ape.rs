use actix_web::web::Json;

use crate::routes::ape::{ApeIndexInput, ApeIndexOutput};

fn ape_index(h: u32, w: u32) -> f32 {
    w as f32 / h as f32
}

pub fn ape_index_from_json(ape_data: Json<ApeIndexInput>) -> ApeIndexOutput {
    ApeIndexOutput {
        height: ape_data.height,
        wingspan: ape_data.wingspan,
        ape_index: ape_index(ape_data.height, ape_data.wingspan)
    }
}

#[cfg(test)]
mod tests_ape_lib {
    use super::*;

    #[test]
    fn test_ape_index() {
        assert_eq!(ape_index(100, 106), 1.06);
    }
}
