use crate::mesh::{Face, FaceDirection, Corner, MeshResult};
use std::cmp::Ordering;
use crate::Chunk;

pub fn join_faces<const SIZE: usize>(faces: Vec<Face<SIZE>>) -> Vec<Face<SIZE>> {
    let rows = join_rows(faces);
    join_columns(rows)
}

fn join_rows<const SIZE: usize>(mut faces: Vec<Face<SIZE>>) -> Vec<Face<SIZE>> {
    let mut faces = faces.into_iter().map(|f| Some(f)).collect::<Vec<_>>();
    let mut rows = vec![];

    while let Some(base_face_opt) = faces.pop() {
        if let Some(base_face) = base_face_opt {
            let mut current_face = base_face;
            while let Some(next_face) = faces.iter_mut().find(|face| {
                if let Some(f) = face {
                    current_face.can_merge_row(f)
                } else {
                    false
                }
            }) {
                current_face.try_merge_face_row(next_face.take().unwrap()).unwrap();
            }

            rows.push(current_face)
        }
    }

    rows
}

fn join_columns<const SIZE: usize>(mut faces: Vec<Face<SIZE>>) -> Vec<Face<SIZE>> {
    let mut columns = vec![];
    while let Some(base_face) = faces.pop() {
        let mut current_face = base_face;
        while let Some((index, _next_face)) = faces.iter().enumerate().find(|(i, face)| {
            current_face.can_merge_column(*face)
        }) {
            current_face.try_merge_face_column(faces.remove(index)).unwrap();
        }

        columns.push(current_face)
    }
    columns
}