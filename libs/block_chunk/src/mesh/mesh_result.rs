use crate::mesh::Face;

#[derive(Debug, Clone)]
pub struct MeshResult<T, const SIZE: usize> {
    pub mesh: Vec<Face<T, SIZE>>,
    pub transparent_mesh: Vec<Face<T, SIZE>>,
    pub unhandled: Vec<(usize, usize, usize)>,
}
