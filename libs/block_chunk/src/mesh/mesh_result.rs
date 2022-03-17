use crate::mesh::Face;

#[derive(Debug, Clone)]
pub struct MeshResult<TE: Send + Sync, const SIZE: usize> {
    pub mesh: Vec<Face<TE, SIZE>>,
    pub transparent_mesh: Vec<Face<TE, SIZE>>,
    pub unhandled: Vec<(usize, usize, usize)>,
}
