pub mod master;
pub mod resources;
pub mod state;

pub use crate::master::*;
pub use crate::resources::*;
pub use crate::state::*;

use macroquad::prelude::*;

pub async fn start(master: &mut Master) {
	master.state.initialize();
	loop {
		master.state.update();

		master.state.render(&master.resources);

		next_frame().await
	}
}