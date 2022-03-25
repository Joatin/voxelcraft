use crate::input::user_action::UserAction;
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InputConfig {
    mouse_sensitivity: f64,
    key_mappings: HashMap<u32, UserAction>,
}

impl InputConfig {
    pub fn get_action(&self, scancode: u32) -> Option<&UserAction> {
        self.key_mappings.get(&scancode)
    }

    pub fn mouse_sensitivity(&self) -> f64 {
        self.mouse_sensitivity
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        let mouse_sensitivity = 0.1;

        let mut key_mappings = HashMap::new();

        key_mappings.insert(13, UserAction::MoveForward);
        key_mappings.insert(2, UserAction::MoveRight);
        key_mappings.insert(0, UserAction::MoveLeft);
        key_mappings.insert(1, UserAction::MoveBackward);
        key_mappings.insert(49, UserAction::Jump);
        key_mappings.insert(56, UserAction::Sneak);

        Self {
            mouse_sensitivity,
            key_mappings,
        }
    }
}
