use macroquad::prelude::mouse_position;
use macroquad::prelude::screen_width;
use macroquad::prelude::screen_height;
use macroquad::prelude::Vec2;
use macroquad::prelude::vec2;

#[derive(Copy, Clone)]
pub struct Viewport {
	pub position: Vec2,
	pub(crate) size: Vec2,
	pub(crate) zoom: f32,
}

impl Default for Viewport {
	fn default() -> Self {
		Self {
			position: Vec2::ZERO,
			size: vec2(960.0, 600.0),
			zoom: 1.0,
		}
	}
}

impl Viewport {
	pub fn new(size: Vec2) -> Self {
		Self {
			size,
			..Default::default()
		}
	}

	pub fn get_size(&self) -> Vec2 { self.size }

	pub fn mouse_position(&self) -> Vec2 {
		let mouse_pos = mouse_position();
		let mut mouse_pos = vec2(mouse_pos.0, mouse_pos.1);
	
		mouse_pos.x = (mouse_pos.x - screen_width() * 0.5) / self.zoom + self.position.x;
		mouse_pos.y = (mouse_pos.y - screen_height() * 0.5) / self.zoom + self.position.y;

		mouse_pos
	}
}