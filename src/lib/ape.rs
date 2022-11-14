use actix_web::web::Json;

use crate::lib::structs::ApeIndexInput;
use crate::lib::structs::ApeIndexOutput;

// height and wingspan can be signed ints, the validator macro will handle input filtering
fn ape_index(height: i16, wingspan: i16) -> f32 {
    wingspan as f32 / height as f32
}

pub fn ape_index_from_json(ape_data: Json<ApeIndexInput>) -> ApeIndexOutput {
    ApeIndexOutput {
        height: ape_data.height,
        wingspan: ape_data.wingspan,
        ape_index: ape_index(ape_data.height, ape_data.wingspan),
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
