use glium::glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, DeviceEvent, MouseButton};
use std::collections::HashSet;
use std::time::{Duration, Instant};

pub struct Input {
    pressed_keys: HashSet<VirtualKeyCode>,
    mouse_sensitivity: f32,
    mouse_clicked: bool,
    last_click_time: Instant,
    click_delay: Duration,
}

impl Input {
    pub fn new() -> Self {
        Input {
            pressed_keys: HashSet::new(),
            mouse_sensitivity: 0.005,
            mouse_clicked: false,
            last_click_time: Instant::now(),
            click_delay: Duration::from_millis(500), // 500ms delay between removals
        }
    }

    pub fn can_remove_cube(&mut self) -> bool {
        if self.mouse_clicked {
            if self.last_click_time.elapsed() >= self.click_delay {
                self.last_click_time = Instant::now();
                return true;
            }
        } else {
            self.last_click_time = Instant::now(); // Reset the click time if mouse is not clicked
        }
        false
    }

    pub fn process_event(&mut self, event: &Event<()>, yaw: &mut f32, pitch: &mut f32) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => { self.pressed_keys.insert(key); },
                            ElementState::Released => { self.pressed_keys.remove(&key); },
                        }
                    }
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    if *button == MouseButton::Left {
                        self.mouse_clicked = *state == ElementState::Pressed;
                    }
                }
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    let (delta_x, delta_y) = *delta;
                    *yaw += delta_x as f32 * self.mouse_sensitivity;
                    *pitch -= delta_y as f32 * self.mouse_sensitivity;
                    *pitch = pitch.clamp(-1.57, 1.57);
                },
                _ => (),
            },
            _ => (),
        }
    }

    pub fn process_input(&self, elapsed: Duration, player_position: &mut [f32; 3], yaw: &mut f32, _pitch: &mut f32) {
        let move_speed = 2.0;
        let move_distance = move_speed * elapsed.as_secs_f32();

        let mut forward = nalgebra::Vector3::new(yaw.sin(), 0.0, -yaw.cos());
        forward.normalize_mut();

        let mut right = nalgebra::Vector3::new(forward.z, 0.0, -forward.x);
        right.normalize_mut();

        if self.pressed_keys.contains(&VirtualKeyCode::A) {
            player_position[0] += forward.x * move_distance;
            player_position[2] += forward.z * move_distance;
        }
        if self.pressed_keys.contains(&VirtualKeyCode::D) {
            player_position[0] -= forward.x * move_distance;
            player_position[2] -= forward.z * move_distance;
        }
        if self.pressed_keys.contains(&VirtualKeyCode::W) {
            player_position[0] -= right.x * move_distance;
            player_position[2] -= right.z * move_distance;
        }
        if self.pressed_keys.contains(&VirtualKeyCode::S) {
            player_position[0] += right.x * move_distance;
            player_position[2] += right.z * move_distance;
        }
        if self.pressed_keys.contains(&VirtualKeyCode::Space) {
            player_position[1] += move_distance;
        }
        if self.pressed_keys.contains(&VirtualKeyCode::LShift) {
            player_position[1] -= move_distance;
        }
    }
}