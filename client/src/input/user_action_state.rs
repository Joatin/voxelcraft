use winit::event::ElementState;

#[derive(Debug, Clone)]
pub enum UserActionState {
    Started,
    Stopped,
}

impl From<ElementState> for UserActionState {
    fn from(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => Self::Started,
            ElementState::Released => Self::Stopped,
        }
    }
}
