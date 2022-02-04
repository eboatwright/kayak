use kayak::UpdateStatus;
use crate::pipes::*;
use crate::bird::*;
use macroquad::prelude::*;
use crate::Resources;
use kayak::State;
use kayak::Context;

pub struct GameState {
    pub score: u16,
    pub game_over: bool,
    pub speed: f32,
    pub pipes: Vec<Pipe>,
    pub bird: Bird,
}

impl State for GameState {
    fn update(&mut self, context: &mut Context) -> UpdateStatus {
        pipes_update(self);
        bird_update(self, context);

        UpdateStatus::Ok
    }

    fn render(&self, context: &Context) {
        let resources: &Resources = context.get_resources();

        background_render(resources);
        pipes_render(self, resources);
        bird_render(self, resources);
        score_render(self);
        instructions_render();
        game_over_render(self);
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            game_over: false,
            speed: 2.0,
            pipes: vec![
                create_pipe(1),
                create_pipe(2),
            ],
            bird: Bird {
                transform: Rect {
                    x: -70.0,
                    y: -31.0,
                    w: 20.0,
                    h: 19.0,
                },
                ..Default::default()
            },
        }
    }
}

pub fn background_render(resources: &Resources) {
    draw_texture(
        resources.background,
        -160.0,
        -100.0,
        WHITE,
    );
}

pub fn score_render(game_state: &GameState) {
    let text = &format!("{}", game_state.score);
    draw_text(
        text,
        0.0 - (text.len() as f32 * 12.0) - 1.0,
        (f32::sin(get_time() as f32 * 2.0) * 5.0).round() - 67.0,
        48.0,
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.5,
        },
    );
    draw_text(
        text,
        0.0 - (text.len() as f32 * 12.0),
        (f32::sin(get_time() as f32 * 2.0) * 5.0).round() - 68.0,
        48.0,
        WHITE,
    );
}

pub fn instructions_render() {
    draw_text(
        "Click / Space - Flap",
        -158.0,
        82.0,
        16.0,
        WHITE,
    );
    draw_text(
        "R - Toggle Rotation",
        -158.0,
        98.0,
        16.0,
        WHITE,
    );
}

pub fn game_over_render(game_state: &GameState) {
    if game_state.game_over {
        draw_text(
            "~ Game Over! ~",
            -148.0,
            (f32::sin(get_time() as f32 * 2.0) * 10.0).round() - 7.0,
            48.0,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            },
        );
        draw_text(
            "~ Game Over! ~",
            -147.0,
            (f32::sin(get_time() as f32 * 2.0) * 10.0).round() - 8.0,
            48.0,
            WHITE,
        );
    }
}