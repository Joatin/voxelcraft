#[derive(Debug, Clone)]
pub struct BlockDescriptor<TE: Send + Sync> {
    pub is_standard_square: bool,
    pub is_transparent: bool,
    pub texture_id: TE,
}
