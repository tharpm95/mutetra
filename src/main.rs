use glium::{Display, Surface, Program, implement_vertex, uniform};
use glium::glutin::{event::{Event, WindowEvent, VirtualKeyCode, ElementState}, event_loop::{ControlFlow, EventLoop}, ContextBuilder, GlProfile, GlRequest, window::WindowBuilder};
use nalgebra::{Matrix4, Perspective3};
use glium::glutin::event::DeviceEvent; // Import for capturing mouse motion events

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Mutetra");
    let context = ContextBuilder::new().with_gl(GlRequest::Latest).with_gl_profile(GlProfile::Core);
    let display = Display::new(window, context, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
    }

    implement_vertex!(Vertex, position);

    let shape = vec![
        Vertex { position: [-0.5, -0.5, -0.5] },
        Vertex { position: [ 0.5, -0.5, -0.5] },
        Vertex { position: [ 0.5,  0.5, -0.5] },
        Vertex { position: [-0.5,  0.5, -0.5] },
        // Other cube faces...
    ];

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let program = Program::from_source(&display, "
        #version 140
        in vec3 position;
        uniform mat4 matrix;
        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    ", "
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    ", None).unwrap();

    let mut player_position = [0.0, 0.0, -2.0_f32];
    let perspective = Perspective3::new(4.0 / 3.0, std::f32::consts::FRAC_PI_2, 0.1, 1024.0);

    // For mouse look
    let mut pitch: f32 = 0.0;
    let mut yaw: f32 = 0.0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if input.state == ElementState::Pressed {
                            match key {
                                VirtualKeyCode::W => player_position[2] += 0.1,
                                VirtualKeyCode::S => player_position[2] -= 0.1,
                                VirtualKeyCode::A => player_position[0] -= 0.1,
                                VirtualKeyCode::D => player_position[0] += 0.1,
                                _ => (),
                            }
                        }
                    }
                },
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    let (delta_x, delta_y) = delta;
                    yaw += delta_x as f32 * 0.005;
                    pitch -= delta_y as f32 * 0.005;
                    pitch = pitch.clamp(-1.57, 1.57);
                },
                _ => (),
            },
            _ => (),
        }

        let direction = nalgebra::Vector3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos()
        ).normalize();

        let view = Matrix4::look_at_rh(
            &nalgebra::Point3::new(player_position[0], player_position[1], player_position[2]),
            &(nalgebra::Point3::from(nalgebra::Vector3::from(player_position) + direction)),
            &nalgebra::Vector3::y_axis(),
        );

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: <[[f32; 4]; 4]>::from(perspective.to_homogeneous() * view),
        };

        target.draw(&glium::VertexBuffer::new(&display, &shape).unwrap(), &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    });
}