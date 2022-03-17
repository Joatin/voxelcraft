use crate::open_gl_to_wgpu_matrix::OPENGL_TO_WGPU_MATRIX;
use cgmath::{
    Deg, Euler, InnerSpace, Matrix4, PerspectiveFov, Point3, Quaternion, Rad, Vector3, Vector4,
};

#[derive(Debug, Clone)]
pub struct FpsCamera {
    pub perspective: PerspectiveFov<f32>,
    pub position: Point3<f32>,
    pub yaw: Deg<f32>,
    pub pitch: Deg<f32>,
}

impl FpsCamera {
    pub fn new<V: Into<Point3<f32>>>(
        position: V,
        yaw: Deg<f32>,
        pitch: Deg<f32>,
        width: f32,
        height: f32,
    ) -> Self {
        let perspective = PerspectiveFov {
            fovy: Deg(70.0).into(),
            aspect: width / height,
            near: 0.1,
            far: 1000.0,
        };

        Self {
            perspective,
            position: position.into(),
            yaw,
            pitch,
        }
    }

    pub fn set_fovy(&mut self, fovy: Deg<f32>) {
        self.perspective.fovy = fovy.into()
    }

    pub fn increase_yaw(&mut self, delta: f64, scale_factor: f32) {
        self.yaw += Deg(delta as f32 * scale_factor);
    }

    pub fn increase_pitch(&mut self, delta: f64, scale_factor: f32) {
        self.pitch += Deg(delta as f32 * scale_factor);
        if self.pitch < Deg(-89.0) {
            self.pitch = Deg(-89.0);
        } else if self.pitch > Deg(89.0) {
            self.pitch = Deg(90.0);
        }
    }

    pub fn set_aspect_ratio(&mut self, width: f32, height: f32) {
        self.perspective.aspect = width / height;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let perspective: Matrix4<f32> = self.perspective.into();

        let position = Vector3 {
            x: self.position.x,
            y: self.position.y * -1.0,
            z: self.position.z,
        };
        let euler = Euler {
            x: self.pitch,
            y: self.yaw,
            z: Deg(0.0),
        };

        OPENGL_TO_WGPU_MATRIX
            * (perspective * (Matrix4::from(euler) * Matrix4::from_translation(position)))
    }
}
