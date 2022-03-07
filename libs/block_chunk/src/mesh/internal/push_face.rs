use crate::mesh::{Face, BlockDescriptor};

pub fn push_face<const SIZE: usize>(mesh: &mut Vec<Face<SIZE>>, transparent_mesh: &mut Vec<Face<SIZE>>, descriptor: &BlockDescriptor, face: Face<SIZE>) {
    if descriptor.is_transparent {
        transparent_mesh.push(face)
    } else {
        mesh.push(face)
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::internal::push_face::push_face;
    use crate::mesh::{BlockDescriptor, Face};
    use crate::BlockOffset;

    #[test]
    fn it_should_push_to_transparent_list_if_transparent() {
        let mut mesh = vec![];
        let mut transparent_mesh = vec![];
        let descriptor = BlockDescriptor {
            is_standard_square: true,
            is_transparent: true
        };

        push_face(&mut mesh, &mut transparent_mesh, &descriptor, Face::north(&BlockOffset::<16>::default()));

        assert_eq!(mesh.len(), 0);
        assert_eq!(transparent_mesh.len(), 1);
    }

    #[test]
    fn it_should_push_to_mesh_vec_if_not_transparent() {
        let mut mesh = vec![];
        let mut transparent_mesh = vec![];
        let descriptor = BlockDescriptor {
            is_standard_square: true,
            is_transparent: false
        };

        push_face(&mut mesh, &mut transparent_mesh, &descriptor, Face::north(&BlockOffset::<16>::default()));

        assert_eq!(mesh.len(), 1);
        assert_eq!(transparent_mesh.len(), 0);
    }
}