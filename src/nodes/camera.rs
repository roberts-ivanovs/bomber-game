use macroquad::{
    experimental::scene::{self, RefMut},
    prelude::*,
};

pub struct Camera {
    bounds: Rect,
    viewport_height: f32,
    macroquad_camera: Camera2D,
}

impl Camera {
    const BUFFER_CAPACITY: usize = 20;

    pub fn new(bounds: Rect, viewport_height: f32) -> Camera {
        Camera {
            bounds,
            viewport_height,
            macroquad_camera: Camera2D::default(),
        }
    }
}

impl Camera {
    pub fn macroquad_camera(&self) -> &Camera2D {
        &self.macroquad_camera
    }
}

impl scene::Node for Camera {
    fn update(mut node: RefMut<Self>) {
        let pos = vec2(10., 10.);

        let aspect = screen_width() / screen_height();

        let viewport_width = node.viewport_height * aspect;

        node.macroquad_camera = Camera2D {
            zoom: vec2(
                1.0 / viewport_width as f32 * 5.,
                -1.0 / node.viewport_height as f32 * 5.,
            ),
            target: vec2(pos.x, pos.y),
            ..Default::default()
        };
        scene::set_camera(*node.macroquad_camera());
    }
}
