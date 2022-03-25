use crate::event::WorldEvent;
use cgmath::{vec3, Array, Deg, Euler, Matrix4, Quaternion, Rotation, Vector3, VectorSpace};
use tokio::sync::broadcast;
use uuid::Uuid;
use voxelcraft_core::entity::EntityPosition;
use voxelcraft_mod::{Entity, LivingEntity};

#[derive(Debug)]
pub struct Player {
    id: Uuid,
    position: EntityPosition,
    world_event_sender: broadcast::Sender<WorldEvent>,
    yaw: Deg<f32>,
    pitch: Deg<f32>,
    velocity: Vector3<f32>,

    is_moving_forward: bool,
    is_moving_backward: bool,
    is_moving_left: bool,
    is_moving_right: bool,
    is_jumping: bool,
    is_sneaking: bool,
}

impl Player {
    pub fn new(
        id: Uuid,
        position: EntityPosition,
        world_event_sender: broadcast::Sender<WorldEvent>,
    ) -> Self {
        let yaw = Deg(0.0);
        let pitch = Deg(0.0);
        let is_moving_forward = false;
        let is_moving_backward = false;
        let is_moving_left = false;
        let is_moving_right = false;
        let is_jumping = false;
        let is_sneaking = false;
        let velocity = vec3(0.0, 0.0, 0.0);
        Self {
            id,
            position,
            world_event_sender,
            yaw,
            pitch,
            is_moving_forward,
            velocity,
            is_moving_backward,
            is_moving_left,
            is_moving_right,
            is_jumping,
            is_sneaking,
        }
    }

    pub fn start_move_forward(&mut self) {
        self.is_moving_forward = true;
    }

    pub fn stop_move_forward(&mut self) {
        self.is_moving_forward = false;
    }

    pub fn start_move_backward(&mut self) {
        self.is_moving_backward = true;
    }

    pub fn stop_move_backward(&mut self) {
        self.is_moving_backward = false;
    }

    pub fn start_move_left(&mut self) {
        self.is_moving_left = true;
    }

    pub fn stop_move_left(&mut self) {
        self.is_moving_left = false;
    }

    pub fn start_move_right(&mut self) {
        self.is_moving_right = true;
    }

    pub fn stop_move_right(&mut self) {
        self.is_moving_right = false;
    }

    pub fn start_jumping(&mut self) {
        self.is_jumping = true;
    }

    pub fn stop_jumping(&mut self) {
        self.is_jumping = false;
    }

    pub fn start_sneaking(&mut self) {
        self.is_sneaking = true;
    }

    pub fn stop_sneaking(&mut self) {
        self.is_sneaking = false;
    }

    pub fn set_head_rotation(&mut self, pitch: Deg<f32>, yaw: Deg<f32>) {
        self.pitch = pitch;
        self.yaw = yaw;
    }

    fn get_max_horizontal_velocity(&self) -> f32 {
        10.0
    }

    fn get_next_forward_velocity(&self, delta: f64) -> Vector3<f32> {
        if self.is_moving_forward {
            vec3(0.0, 0.0, 64.0 * delta as f32)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }

    fn get_next_backward_velocity(&self, delta: f64) -> Vector3<f32> {
        if self.is_moving_backward {
            vec3(0.0, 0.0, -64.0 * delta as f32)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }

    fn get_next_right_velocity(&self, delta: f64) -> Vector3<f32> {
        if self.is_moving_right {
            vec3(64.0 * delta as f32, 0.0, 0.0)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }

    fn get_next_left_velocity(&self, delta: f64) -> Vector3<f32> {
        if self.is_moving_left {
            vec3(-64.0 * delta as f32, 0.0, 0.0)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }

    fn get_next_jump_velocity(&self, delta: f64) -> Vector3<f32> {
        if self.is_jumping {
            vec3(0.0, -64.0 * delta as f32, 0.0)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }

    fn get_next_sneak_velocity(&self, delta: f64) -> Vector3<f32> {
        if self.is_sneaking {
            vec3(0.0, 64.0 * delta as f32, 0.0)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }

    fn limit_velocity(&self, mut velocity: Vector3<f32>) -> Vector3<f32> {
        if velocity.x > self.get_max_horizontal_velocity() {
            velocity.x = self.get_max_horizontal_velocity()
        }
        if velocity.x < self.get_max_horizontal_velocity() * -1.0 {
            velocity.x = self.get_max_horizontal_velocity() * -1.0
        }

        if velocity.z > self.get_max_horizontal_velocity() {
            velocity.z = self.get_max_horizontal_velocity()
        }
        if velocity.z < self.get_max_horizontal_velocity() * -1.0 {
            velocity.z = self.get_max_horizontal_velocity() * -1.0
        }

        if velocity.y > self.get_max_horizontal_velocity() {
            velocity.y = self.get_max_horizontal_velocity()
        }
        if velocity.y < self.get_max_horizontal_velocity() * -1.0 {
            velocity.y = self.get_max_horizontal_velocity() * -1.0
        }

        velocity
    }

    fn deaccelerate_velocity(&mut self, delta: f64) {
        self.velocity = self.velocity.lerp(vec3(0.0, 0.0, 0.0), delta as f32 * 8.0);
    }
}

#[async_trait::async_trait]
impl Entity for Player {
    fn id(&self) -> Uuid {
        self.id
    }

    fn position(&self) -> &EntityPosition {
        &self.position
    }

    async fn update_position(&mut self, delta: f64) {
        self.deaccelerate_velocity(delta);
        let next_velocity = self.get_next_forward_velocity(delta)
            + self.get_next_backward_velocity(delta)
            + self.get_next_right_velocity(delta)
            + self.get_next_left_velocity(delta)
            + self.get_next_jump_velocity(delta)
            + self.get_next_sneak_velocity(delta);
        let next_velocity = self.limit_velocity(next_velocity);

        let angle = Quaternion::from(Euler::<Deg<f32>> {
            x: Deg(180.0),
            y: self.yaw + Deg(180.0),
            z: Deg(0.0),
        });

        let rotated_velocity = angle.rotate_vector(next_velocity);

        self.velocity += rotated_velocity;

        self.position.offset.x += self.velocity.x * delta as f32;
        self.position.offset.y += self.velocity.y * delta as f32;
        self.position.offset.z += self.velocity.z * delta as f32;
    }
}

impl LivingEntity for Player {}
