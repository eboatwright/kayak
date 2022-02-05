use crate::Context;
use std::collections::HashMap;
use macroquad::prelude::*;

// This holds data that can be looped through and used as a tilemap
pub struct Map {
	pub tile_size: u16,
	pub tiles: Vec<Vec<u16>>,
	// This holds the Y texture source position of each tile
	pub y_source_offset: HashMap<u16, f32>,

	pub texture: Texture2D,
}

impl Default for Map {
	fn default() -> Self {
		Self {
			tile_size: 16,
			tiles: Vec::new(),
			y_source_offset: HashMap::new(),

			texture: Texture2D::empty(),
		}
	}
}

impl Map {
	pub fn render(&self, context: &Context) {
		// This loops through each tile to get x and y coordinates
		for y in 0..self.tiles.len() {
			for x in 0..self.tiles[y].len() {
				// Zero means nothing so skip it
				if self.tiles[y][x] == 0
				// Check if the tile position is even on screen. Because if it's not, we don't need to render it
				|| x as f32 * (self.tile_size as f32) < context.viewport.left()
				|| x as f32 * (self.tile_size as f32) > context.viewport.right()
				|| y as f32 * (self.tile_size as f32) < context.viewport.top()
				|| y as f32 * (self.tile_size as f32) > context.viewport.bottom() {
					continue;
				}
				draw_texture_ex(
					self.texture,
					x as f32 * self.tile_size as f32,
					y as f32 * self.tile_size as f32,
					WHITE,
					DrawTextureParams {
						source: Some(Rect {
							x: self.tiles[y][x] as f32,
							y: match self.y_source_offset.get(&self.tiles[y][x]) {
								Some(y_offset) => *y_offset,
								None => 0.0,
							},
							w: self.tile_size as f32,
							h: self.tile_size as f32,
						}),
						..Default::default()
					},
				);
			}
		}
	}
}