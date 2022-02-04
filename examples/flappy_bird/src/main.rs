mod bird;
mod game_state;
mod pipes;
mod resources;

use macroquad::audio::load_sound;
use kayak::Viewport;
use crate::game_state::GameState;
use kayak::start;
use crate::resources::Resources;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_string(),
        window_width: 960,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let bird_tex = load_texture("res/tex/bird.png").await.unwrap();
    bird_tex.set_filter(FilterMode::Nearest);

    let state = Box::new(GameState::default());

    let viewport = Viewport::new(vec2(320.0, 200.0));

    let resources = Box::new(Resources {
        background: load_texture("res/tex/background.png").await.unwrap(),
        pipes: load_texture("res/tex/pipes.png").await.unwrap(),
        bird: bird_tex,

        score: load_sound("res/sfx/score.ogg").await.unwrap(),
        game_over: load_sound("res/sfx/game_over.ogg").await.unwrap(),
        flap: load_sound("res/sfx/flap.ogg").await.unwrap(),
    });

    start(state, viewport, resources).await;
}