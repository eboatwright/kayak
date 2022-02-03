use std::any::Any;
use macroquad::prelude::*;
use kayak::*;

// See Macroquad's docs.rs page for more on rendering and window configuration
// NOTE: Some things will be different, as this project uses my modified version of macroquad.
// https://github.com/eboatwright/macroquad/
// https://docs.rs/macroquad/0.3.13/macroquad/
#[macroquad::main("Hello Kayak!")]
async fn main() {
	let mut master = Master {
		state: Box::new(GameState {
			// Just put empty values here because initialize will get called on it
			player_score: 0,
		}),
		resources: Box::new(Resources {
			// Load all your textures, sounds and fonts here
			// example_texture: load_texture("res/img/this_texture_doesnt_exist.png").await.unwrap(),
		}),
		// Or if your game doesn't have any textures, sounds or fonts just do
		// ..Default::default()
	};

	let viewport = Viewport::new(vec2(960.0, 600.0));
	
	start(&mut master, viewport).await;
}

// Put all your game variables here
struct GameState {
	player_score: u16,
}

// Implement all state functions
impl State for GameState {
	fn initialize(&mut self) {
		// Will be called when the state is loaded
		println!("GameState Initialize!");
	}

	fn update(&mut self, _viewport: &mut Viewport) {
		// Will be called every frame (only on the current scene in Master)
		println!("GameState Update!");
		self.player_score += 1;
	}

	fn render(&self, _viewport: &Viewport, _resources: &Box<dyn ResourceContainer>) {
		// Will be called every frame after update (only on the current scene in Master)
		// This converts the Box trait into your Resources struct
		// Replace this with your Resources struct's name
		//                     |
		// let resources: &Resources = convert_resources(resources);
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