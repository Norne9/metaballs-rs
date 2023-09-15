use std::collections::VecDeque;

use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
    Ui,
};

use crate::world::World;

const MAX_FRAME_TIMES: usize = 256;

pub struct Gui {
    mouse_position: Vec2,
    show_window: bool,
    choice: Option<usize>,
    show_debug_info: bool,
    frame_times: VecDeque<f32>,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            mouse_position: vec2(0.0, 0.0),
            show_window: false,
            choice: Some(0),
            show_debug_info: false,
            frame_times: VecDeque::with_capacity(MAX_FRAME_TIMES),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let (mx, my) = mouse_position();
        self.mouse_position = vec2(mx, my);

        if mx < 10.0 && my < 10.0 {
            self.show_window = true;
        }

        self.frame_times.push_front(dt);
        while self.frame_times.len() > MAX_FRAME_TIMES {
            self.frame_times.pop_back();
        }
    }

    pub fn draw(&mut self, world: &mut World) {
        if !self.show_window {
            return;
        }

        widgets::Window::new(hash!(), vec2(30., 30.), vec2(340., 205.))
            .label("Settings")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &format!("Ball count: {}", world.len()));
                ui.slider(hash!(), "World speed", 0.1f32..3.0f32, &mut world.speed);
                let mut zoom = 1.0 / world.zoom;
                ui.slider(hash!(), "Zoom", 0.5f32..2.0f32, &mut zoom);
                world.zoom = 1.0 / zoom;
                ui.separator();

                ui.combo_box(
                    hash!(),
                    "Click action",
                    &["Add ball", "Remove ball"],
                    &mut self.choice,
                );
                if ui.button(None, "Restart simulation") {
                    world.restart();
                }
                if ui.button(None, "Hide settings") {
                    self.show_window = false;
                }
                ui.separator();

                ui.checkbox(hash!(), "Show debug info", &mut self.show_debug_info);
                if self.show_debug_info {
                    let avg_dt = self.frame_times.iter().fold(0.0, |acc, e| acc + e)
                        / (self.frame_times.len() as f32);
                    ui.label(None, &format!("FPS: {:.1}", 1.0 / avg_dt));
                    self.draw_times_chart(ui);
                }
                ui.separator();
            });
    }

    fn draw_times_chart(&self, ui: &mut Ui) {
        const CHART_HEIGHT: f32 = 20.0;

        let (min_dt, max_dt) = self
            .frame_times
            .iter()
            .fold((1.0, 0.0), |(mn, mx), e| (e.min(mn), e.max(mx)));

        ui.same_line(64.0);
        let mut canvas = ui.canvas();
        let cursor = canvas.cursor();
        canvas.rect(
            Rect::new(cursor.x, cursor.y, MAX_FRAME_TIMES as f32, CHART_HEIGHT),
            None,
            DARKGRAY,
        );

        for (x, dt) in self
            .frame_times
            .iter()
            .map(|e| (e - min_dt) / (max_dt - min_dt))
            .enumerate()
        {
            let x = x as f32;
            canvas.line(
                vec2(x, CHART_HEIGHT) + cursor,
                vec2(x, (1.0 - dt) * CHART_HEIGHT) + cursor,
                GREEN,
            );
        }
    }

    pub fn mouse_in_window(&self) -> bool {
        root_ui().is_mouse_over(self.mouse_position)
    }
    pub fn click_to_add(&self) -> bool {
        self.choice == Some(0)
    }
    pub fn show_debug_info(&self) -> bool {
        self.show_debug_info
    }
}
