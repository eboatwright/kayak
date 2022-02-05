use macroquad::prelude::Vec2;

// Holds positional data that you can use for rendering or physics
pub struct Transform {
	pub position: Vec2,
	pub rotation: f32,
}