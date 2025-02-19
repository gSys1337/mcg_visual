use eframe::emath;
use egui::{Context, Direction};
use rand::Rng;

#[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
use crate::log;
pub mod card;
pub mod vfx;
use crate::example;
pub use crate::game::card::Card;

pub struct App {
    current_state: Anchor,
    cards: Vec<Box<dyn Card>>,
    next_suit: usize,
    next_rank: usize,
    screen_width: f32,
    screen_height: f32,
    hand: vfx::HandLayout,
    stack: vfx::Stack,
}

impl App {
    #[allow(unused)]
    pub fn new(cc: &eframe::CreationContext) -> Self {
        crate::utils::set_panic_hook();
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut cards: Vec<Box<dyn Card>> = Vec::new();
        // cards.push(Box::new(example::Backside::new()));
        let mut hand: vfx::HandLayout = Default::default();
        for _ in 0..10 {
            hand.add_card(Box::new(example::ConventionalCard::new_random()));
        }
        let mut stack: vfx::Stack = Default::default();
        for _ in 0..10 {
            stack.add_card(Box::new(example::ConventionalCard::new_random()));
        }
        Self {
            current_state: Anchor::Menu,
            cards,
            next_suit: Default::default(),
            next_rank: Default::default(),
            screen_width: 0.0,
            screen_height: 0.0,
            hand,
            stack
        }
    }
}

enum Anchor {
    Menu,
    Game,
    Settings,
}

use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::collections::BTreeMap;

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = 30.0;
            let text_styles: BTreeMap<_, _> = [
                (Heading, FontId::new(size, Proportional)),
                (Name("Heading2".into()), FontId::new(size, Proportional)),
                (Name("Context".into()), FontId::new(size, Proportional)),
                (Body, FontId::new(size, Proportional)),
                (Monospace, FontId::new(size, Proportional)),
                (Button, FontId::new(size, Proportional)),
                (Small, FontId::new(size, Proportional)),
            ]
            .into();

            // Mutate global styles with new text styles
            ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());

            let layout = egui::Layout::from_main_dir_and_cross_align(
                Direction::TopDown,
                emath::Align::Center,
            );
            ui.with_layout(layout, |ui| match self.current_state {
                Anchor::Menu => {
                    let start = egui::Button::new("Start Game");
                    if ui.add(start).clicked() {
                        #[cfg(target_arch = "wasm32")]
                        log("game started");
                        self.current_state = Anchor::Game;
                    };
                    let settings = egui::Button::new("Settings");
                    if ui.add(settings).clicked() {
                        #[cfg(target_arch = "wasm32")]
                        log("configuring textures");
                        self.current_state = Anchor::Settings;
                    }
                }
                Anchor::Game => {
                    let back = egui::Button::new("Back");
                    if ui.add(back).clicked() {
                        #[cfg(target_arch = "wasm32")]
                        log("back to main menu");
                        self.current_state = Anchor::Menu;
                    }
                    ui.add(&mut self.hand);
                    ui.add(&mut self.stack);
                    for (idx, card) in self.cards.iter_mut().enumerate() {
                        let ir = egui::Area::new(egui::Id::new(idx))
                            .sense(egui::Sense::click_and_drag())
                            .current_pos(card.pos())
                            .show(&ctx, |ui| {
                                // important to use ``&**card`` because rust gets it somehow wrong ¯\_(ツ)_/¯
                                // i assume this is an "issue" with deref coercion because
                                // Rust can include `*`s at compile-time but no `&`
                                ui.add(&**card)
                            });
                        let r = ir.inner;
                        if r.is_pointer_button_down_on() {
                            card.translate(r.drag_delta());
                        }
                    }
                }
                Anchor::Settings => {
                    let back = egui::Button::new("Back");
                    self.screen_width = ctx.available_rect().width();
                    self.screen_height = ctx.available_rect().height();
                    if ui.add(back).clicked() {
                        #[cfg(target_arch = "wasm32")]
                        log("back to main menu");
                        self.current_state = Anchor::Menu;
                    }
                    let suit_area = egui::Area::new(egui::Id::new("suit"))
                        .sense(egui::Sense::click())
                        .current_pos(ui.next_widget_position());
                    let ir = suit_area.show(ctx, |ui| {
                        egui::ComboBox::from_label("Suit").show_index(
                            ui,
                            &mut self.next_suit,
                            example::Suit::len(),
                            |idx| format!("{}", example::Suit::from(idx)),
                        )
                    });
                    ui.advance_cursor_after_rect(ir.response.rect);
                    let rank_area = egui::Area::new(egui::Id::new("rank"))
                        .sense(egui::Sense::click())
                        .current_pos(ui.next_widget_position());
                    let ir = rank_area.show(ctx, |ui| {
                        egui::ComboBox::from_label("Rank").show_index(
                            ui,
                            &mut self.next_rank,
                            example::Rank::len(),
                            |idx| format!("{}", example::Rank::from(idx)),
                        )
                    });
                    ui.advance_cursor_after_rect(ir.response.rect);
                    if ui.button("Add").clicked() {
                        let x = rand::thread_rng().gen_range(0..self.screen_width as i32) as f32;
                        let y = rand::thread_rng().gen_range(0..self.screen_height as i32) as f32;
                        let card = example::ConventionalCard {
                            suit: example::Suit::from(self.next_suit),
                            rank: example::Rank::from(self.next_rank),
                            pos: egui::Pos2::from((x, y)),
                        };
                        self.cards.push(Box::new(card));
                        #[cfg(target_arch = "wasm32")]
                        log(format!("added card @ ({}|{})", x, y).as_str());
                    }
                    if ui.button("Random").clicked() {
                        self.hand
                            .add_card(Box::new(example::ConventionalCard::new_random()));
                    }
                }
            });
        });
    }
}
