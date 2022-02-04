use std::any::Any;
use macroquad::prelude::*;
use kayak::*;

// See Macroquad's docs.rs page for more on rendering and window configuration
// NOTE: Some things will be different, as this project uses my modified version of macroquad.
// https://github.com/eboatwright/macroquad/
// https://docs.rs/macroquad/0.3.13/macroquad/
#[macroquad::main("Hello Kayak!")]
async fn main() {
	// Put the rendering pixel size in here |
	// So if you want 3 screen pixels to be 1 game pixel, divide your starting window size by 3 and put it here
	let viewport = Viewport::new(vec2(960.0, 600.0));

	let resources = Box::new(Resources {
		// Load all your textures, sounds and fonts here
		// example_texture: load_texture("res/img/this_texture_doesnt_exist.png").await.unwrap(),
	});
	
	start(Box::new(GameState::default()), viewport, resources).await;
}

// Put all your game variables here
struct GameState {
	player_score: u16,
}

impl Default for GameState {
	// This is basically the initialize function
	fn default() -> Self {
		Self {
			player_score: 0,
		}
	}
}

// Implement all state functions
impl State for GameState {
	fn update(&mut self, _context: &mut Context) -> UpdateStatus {
		// Will be called every frame (only on the current scene in Master)
		println!("GameState Update!");
		self.player_score += 1;

		UpdateStatus::Ok
	}

	fn render(&self, _context: &Context) {
		// Will be called every frame after update (only on the current scene in Master)
		// This get's the current resources type
		// Replace this with your Resources struct's name
		//                     |
		// let resources: &Resources = context.get_resources();
		println!("GameState Render!");
		draw_text(
			&format!("Player Score: {}", self.player_score),
			20.0,
			20.0,
			16.0,
			WHITE,
		);
	}
}

// Put all your textures, sounds and fonts in here
struct Resources {
	// example_texture: Texture2D,
}

// This is just a marker trait
impl ResourceContainer for Resources {
	fn as_any(&self) -> &dyn Any { self as &dyn Any }
}