use uuid::Uuid;

pub trait Dimension {
    fn id(&self) -> Uuid;
    fn name(&self) -> &str;
}