use glam::*;

pub enum Direction {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

#[derive(Default)]
pub struct Camera {
    position: Vec3,
    direction: Vec3,
    right: Vec3,
    yaw: f32,
    pitch: f32,
    pub view: Mat4,
}

impl Camera {
    pub fn new() -> Camera {
        let position = Vec3::new(0.0, 0.0, 0.0);
        let pitch = 0.0;
        let yaw = 0.0;

        let mut camera = Camera {
            position,
            yaw,
            pitch,
            ..Default::default()
        };

        camera.update();
        camera
    }

    pub fn update(&mut self) {
        self.direction = Vec3::new(
            f32::cos(f32::to_radians(self.pitch)) * f32::cos(f32::to_radians(self.yaw)),
            f32::sin(f32::to_radians(self.pitch)),
            f32::cos(f32::to_radians(self.pitch)) * f32::sin(f32::to_radians(self.yaw)),
        );

        self.right = self.direction.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = self.right.cross(self.direction).normalize();

        self.view = Mat4::look_at_rh(self.position, self.position + self.direction, up)
    }

    pub fn process_keyboard(&mut self, direction: Direction, delta_time: f32) {
        const SPEED: f32 = 10.0;
        let speed = SPEED * delta_time;
        match direction {
            Direction::FORWARD => {
                self.position += self.direction * speed;
            }
            Direction::BACKWARD => {
                self.position -= self.direction * speed;
            }
            Direction::LEFT => {
                self.position -= self.right * speed;
            }
            Direction::RIGHT => {
                self.position += self.right * speed;
            }
        }
    }

    pub fn process_mouse(&mut self, x_offset: f32, y_offset: f32) {
        const SENSITIVITY: f32 = 0.1;

        let x_offset = x_offset * SENSITIVITY;
        let y_offset = y_offset * SENSITIVITY;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if self.pitch < -89.9 {
            self.pitch = -89.9;
        } else if self.pitch > 89.9 {
            self.pitch = 89.9;
        }
    }
}
