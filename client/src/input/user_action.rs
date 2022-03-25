use voxelcraft_id::ModId;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum UserAction {
    MoveForward,
    MoveBackward,
    MoveRight,
    MoveLeft,
    Jump,
    Sneak,
    Modded { mod_id: ModId, action: String },
}
