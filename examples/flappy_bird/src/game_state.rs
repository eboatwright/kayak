use crate::pipes::*;
use crate::bird::*;
use macroquad::prelude::*;
use crate::Resources;
use kayak::Viewport;
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
    fn initialize(&mut self, context: &mut Context) {
        self.score = 0;

        self.bird.transform = Rect {
            x: -70.0,
            y: -31.0,
            w: 20.0,
            h: 19.0,
        };
        self.bird.rotation = 0.0;
        self.bird.done_game_over_animation = false;
        self.bird.y_velocity = 0.0;
        self.bird.game_over_timer = 10.0;

        self.game_over = false;
        self.bird.started = false;

        self.pipes = vec![
            create_pipe(&context.viewport, 1),
            create_pipe(&context.viewport, 2),
        ];
    }

    fn update(&mut self, context: &mut Context) {
        pipes_update(self, &context.viewport);
        bird_update(self, context);
    }

    fn render(&self, context: &Context) {
        let resources: &Resources = context.get_resources();
        background_render(&context.viewport, resources);
        pipes_render(self, resources);
        bird_render(self, resources);
        score_render(self, &context.viewport);
        instructions_render(&context.viewport);
        game_over_render(self);
    }
}

pub fn background_render(viewport: &Viewport, resources: &Resources) {
    draw_texture(
        resources.background,
        -viewport.screen_size().x * 0.5,
        -viewport.screen_size().y * 0.5,
        WHITE,
    );
}

pub fn score_render(game_state: &GameState, viewport: &Viewport) {
    let text = &format!("{}", game_state.score);
    draw_text(
        text,
        0.0 - (text.len() as f32 * 12.0) - 1.0,
        (f32::sin(get_time() as f32 * 2.0) * 5.0).round() - viewport.screen_size().y * 0.5 + 33.0,
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
        (f32::sin(get_time() as f32 * 2.0) * 5.0).round() - viewport.screen_size().y * 0.5 + 32.0,
        48.0,
        WHITE,
    );
}

pub fn instructions_render(viewport: &Viewport) {
    draw_text(
        "Click / Space - Flap",
        -viewport.screen_size().x * 0.5 + 2.0,
        viewport.screen_size().y * 0.5 - 18.0,
        16.0,
        WHITE,
    );
    draw_text(
        "R - Toggle Rotation",
        -viewport.screen_size().x * 0.5 + 2.0,
        viewport.screen_size().y * 0.5 - 2.0,
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