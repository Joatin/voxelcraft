use crate::interface::pages::Page;
use crate::interface::widget::{Widget, ButtonWidget};
use crate::gpu::Gpu;

pub struct HomePage {
    button: ButtonWidget
}

impl HomePage {
    pub fn new(gpu: &Gpu) -> Self {
        let button = ButtonWidget::new(gpu, "HELLO WORLD", 100.0, 100.0, 40.0);
        Self {
            button
        }
    }
}

impl Page for HomePage {
    fn widgets(&mut self) -> Vec<&mut dyn Widget> {
        vec![
            &mut self.button
        ]
    }
}