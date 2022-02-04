use crate::GameState;
use crate::Resources;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

#[derive(Debug)]
pub struct Pipe {
	pub transform: Rect,
	pub scored: bool,
}

pub fn create_pipe(index: u8) -> Pipe {
	Pipe {
	    transform: Rect {
	        x: 180.0 * index as f32,
	        y: gen_range(-200.0f32, -160.0).round(),
	        w: 31.0,
	        h: 320.0,
	    },
	    scored: false,
	}
}

pub fn pipes_update(game_state: &mut GameState) {
    if game_state.game_over
    || !game_state.bird.started {
        return;
    }
    for i in (0..game_state.pipes.len()).rev() {
        game_state.pipes[i].transform.x -= game_state.speed * delta_time();
        if game_state.pipes[i].transform.right() < -160.0 {
            game_state.pipes[i] = create_pipe(1);
        }
    }
}

pub fn pipes_render(game_state: &GameState, resources: &Resources) {
    for pipe in game_state.pipes.iter() {
        draw_texture(
            resources.pipes,
            pipe.transform.x,
            pipe.transform.y,
            WHITE,
        );
    }
}