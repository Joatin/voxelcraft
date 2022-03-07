use crate::mesh::Face;

#[derive(Debug, Clone)]
pub struct MeshResult<const SIZE: usize> {
    pub mesh: Vec<Face<SIZE>>,
    pub transparent_mesh: Vec<Face<SIZE>>,
    pub unhandled: Vec<(usize, usize, usize)>,
}