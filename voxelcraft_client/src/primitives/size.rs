use winit::dpi::PhysicalSize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

impl From<PhysicalSize<u32>> for Size {
    fn from(size: PhysicalSize<u32>) -> Self {
        Self {
            width: size.width as f32,
            height: size.height as f32,
        }
    }
}

impl From<Size> for PhysicalSize<u32> {
    fn from(size: Size) -> Self {
        Self {
            width: size.width as u32,
            height: size.height as u32,
        }
    }
}

impl From<Size> for iced::Size<f32> {
    fn from(size: Size) -> Self {
        Self {
            width: size.width,
            height: size.height
        }
    }
}