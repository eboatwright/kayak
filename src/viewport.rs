use macroquad::prelude::mouse_position;
use macroquad::prelude::screen_width;
use macroquad::prelude::screen_height;
use macroquad::prelude::Vec2;
use macroquad::prelude::vec2;

// This is basically just a Camera
// I just can't call it Camera, because Macroquad already has that
#[derive(Copy, Clone)]
pub struct Viewport {
	pub position: Vec2,
	pub(crate) screen_size: Vec2,
	pub(crate) zoom: f32,
}

impl Default for Viewport {
	fn default() -> Self {
		Self {
			position: Vec2::ZERO,
			screen_size: vec2(960.0, 600.0),
			zoom: 1.0,
		}
	}
}

impl Viewport {
	pub fn new(screen_size: Vec2) -> Self {
		Self {
			screen_size,
			..Default::default()
		}
	}

	// This is just a get function, because the screen size should not be changed at runtime
	pub fn screen_size(&self) -> Vec2 { self.screen_size }

	pub fn mouse_position(&self) -> Vec2 {
		// Get the full mouse position
		let mut mouse_pos = mouse_position();

		// Scale to game view size                               // Convert to game world space by adding the viewport's position
		mouse_pos.0 = (mouse_pos.0 - screen_width() * 0.5) / self.zoom + self.position.x;
		mouse_pos.1 = (mouse_pos.1 - screen_height() * 0.5) / self.zoom + self.position.y;

		// Convert the (f32, f32) into Vec2
		Vec2::from(mouse_pos)
	}
}