use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;
use uuid::Uuid;

pub struct BiomeCenterPoint {
    x: f64,
    z: f64,
    id: Uuid,
}

impl BiomeCenterPoint {
    pub fn calculate_position(seed: u128, grid_size: f64, pos_x: u64, pos_z: u64) -> Self {
        let mut generator = ChaCha12Rng::from_seed(
            [
                seed.to_ne_bytes(),
                [pos_x.to_ne_bytes(), pos_z.to_ne_bytes()]
                    .concat()
                    .try_into()
                    .unwrap(),
            ]
            .concat()
            .try_into()
            .unwrap(),
        );

        let step = grid_size / u32::MAX as f64;

        let x = step * generator.next_u32() as f64;
        let z = step * generator.next_u32() as f64;

        Self {
            x,
            z,
            id: Uuid::from_bytes(
                [
                    generator.next_u64().to_ne_bytes(),
                    generator.next_u64().to_ne_bytes(),
                ]
                .concat()
                .try_into()
                .unwrap(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::builtin_layers::biomes::biome_center_point::BiomeCenterPoint;

    #[test]
    fn it_should_always_return_the_same_result_for_same_input() {
        let marker1 = BiomeCenterPoint::calculate_position(0, 100.0, 0, 0);
        let marker2 = BiomeCenterPoint::calculate_position(0, 100.0, 0, 0);

        println!("{}", marker1.z);
        println!("{}", marker1.x);

        assert_eq!(marker1.z, marker2.z);
        assert_eq!(marker1.x, marker2.x);
        assert_eq!(marker1.id, marker2.id);
    }

    #[test]
    fn different_seeds_should_yield_different_result() {
        let marker1 = BiomeCenterPoint::calculate_position(0, 100.0, 0, 0);
        let marker2 = BiomeCenterPoint::calculate_position(1, 100.0, 0, 0);

        assert_ne!(marker1.z, marker2.z);
        assert_ne!(marker1.x, marker2.x);
        assert_ne!(marker1.id, marker2.id);
    }
}
