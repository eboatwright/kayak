use crate::extras::Transform;
use macroquad::prelude::Vec2;

// Holds data for collisions
// Offset is distance from top left of the transform. So that, if you're rendering a texture where the transform is,
// You can make the bounding box smaller than the texture
pub struct BoundingBox {
	pub size: Vec2,
	pub offset: Vec2,
}

impl BoundingBox {
	// Basic AABB rectangle collisions
	pub fn overlaps(
		a: (Transform, BoundingBox),
		b: (Transform, BoundingBox),
	) -> bool {
		let a_position = a.0.position + a.1.offset;
		let b_position = b.0.position + b.1.offset;

		   a_position.x < b_position.x + b.1.size.x
		|| a_position.x + a.1.size.x > b_position.x
		|| a_position.y < b_position.y + b.1.size.y
		|| a_position.y + a.1.size.y > b_position.y
	}
}