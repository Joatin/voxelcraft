use crate::mesh::BlockDescriptor;
use crate::{BlockOffset, Chunk};

pub fn should_create_face<
    T: Send + Sync,
    C: Send + Sync + Fn(&T) -> Option<BlockDescriptor>,
    const SIZE: usize,
>(
    chunk: &Chunk<T, SIZE>,
    describe_callback: &C,
    neighbour_position: Option<BlockOffset<SIZE>>,
) -> bool {
    neighbour_position.map_or(true, |neighbour_position| {
        describe_callback(chunk.get(&neighbour_position)).map_or(true, |neighbour_descriptor| {
            !neighbour_descriptor.is_standard_square || neighbour_descriptor.is_transparent
        })
    })
}

#[cfg(test)]
mod tests {
    use crate::mesh::internal::greedy_mesh::should_create_face::should_create_face;
    use crate::mesh::BlockDescriptor;
    use crate::{BlockOffset, Chunk};

    #[test]
    fn it_should_return_true_if_neighbour_position_is_none() {
        assert!(should_create_face::<_, (), _, 4>(
            &Chunk::<u32, 4>::default(),
            &|_| None,
            None
        ));
    }

    #[test]
    fn it_should_return_true_if_describe_callback_returns_none() {
        assert!(should_create_face::<_, (), _, 4>(
            &Chunk::<u32, 4>::default(),
            &|_| None,
            Some(BlockOffset::default())
        ));
    }

    #[test]
    fn it_should_return_false_if_describe_callback_returns_is_standard_square_true() {
        assert!(!should_create_face::<_, _, _, 4>(
            &Chunk::<u32, 4>::default(),
            &|_| Some(BlockDescriptor {
                is_standard_square: true,
                is_transparent: false,
                texture_id: 0
            }),
            Some(BlockOffset::default())
        ));
    }

    #[test]
    fn it_should_return_true_if_describe_callback_returns_is_standard_square_false() {
        assert!(should_create_face::<_, _, _, 4>(
            &Chunk::<u32, 4>::default(),
            &|_| Some(BlockDescriptor {
                is_standard_square: false,
                is_transparent: false,
                texture_id: 0
            }),
            Some(BlockOffset::default())
        ));
    }

    #[test]
    fn it_should_return_true_if_describe_callback_returns_is_transparent_true() {
        assert!(should_create_face::<_, _, _, 4>(
            &Chunk::<u32, 4>::default(),
            &|_| Some(BlockDescriptor {
                is_standard_square: true,
                is_transparent: true,
                texture_id: 0
            }),
            Some(BlockOffset::default())
        ));
    }
}
