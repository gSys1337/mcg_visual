pub mod card;

use crate::game::card::example;
#[cfg(target_arch = "wasm32")]
use crate::log;
use eframe::emath::Align;
use eframe::Frame;
use egui::{Context, Direction};
use rand::Rng;
use std::ops::Add;

pub struct App {
    card_sources: Vec<Vec<egui::ImageSource<'static>>>,
    current_state: Anchor,
    cards: Vec<example::ConventionalCard>,
    next_suit: example::Suit,
    next_rank: example::Rank,
    screen_width: f32,
    screen_height: f32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        crate::utils::set_panic_hook();
        #[cfg(target_arch = "wasm32")]
        log("New App created.");
        Self {
            current_state: Anchor::Menu,
            card_sources: example::ConventionalCard::load_image_sources(),
            cards: vec![],
            next_suit: Default::default(),
            next_rank: Default::default(),
            screen_width: 0.0,
            screen_height: 0.0,
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
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
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

            let layout =
                egui::Layout::from_main_dir_and_cross_align(Direction::TopDown, Align::Center);
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
                    let sources = &self.card_sources;
                    for (idx, card) in self.cards.iter_mut().enumerate() {
                        egui::Area::new(egui::Id::new(idx))
                            .sense(egui::Sense::click_and_drag())
                            .current_pos(card.pos)
                            .show(&ctx, |ui| {
                                let (idx_suit, idx_rank) = card.get_source_index();
                                let card_img =
                                    egui::Image::new(sources[idx_suit][idx_rank].clone())
                                        .sense(egui::Sense::click_and_drag())
                                        .fit_to_original_size(1.0);
                                let resp = ui.add(card_img);
                                if resp.is_pointer_button_down_on() {
                                    let new = card.pos.add(resp.drag_delta());
                                    card.pos = new;
                                    //#[cfg(target_arch = "wasm32")]
                                    //log(format!("card.pos: {:?}", new).as_str());
                                }
                            });
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
                    let ir = egui::Area::new(egui::Id::new("suit"))
                        .sense(egui::Sense::click())
                        .current_pos(ui.next_widget_position())
                        .show(ctx, |ui| {
                            egui::ComboBox::from_label("Suit")
                                .selected_text(format!("{:?}", self.next_suit))
                                .show_ui(ui, |ui| {
                                    for suit in example::Suit::iter() {
                                        ui.selectable_value(
                                            &mut self.next_suit,
                                            suit,
                                            format!("{:?}", suit),
                                        );
                                    }
                                });
                        });
                    ui.advance_cursor_after_rect(ir.response.rect);
                    let ir = egui::Area::new(egui::Id::new("rank"))
                        .sense(egui::Sense::click())
                        .current_pos(ui.next_widget_position())
                        .show(ctx, |ui| {
                            egui::ComboBox::from_label("Rank")
                                .selected_text(format!("{:?}", self.next_rank))
                                .show_ui(ui, |ui| {
                                    for rank in example::Rank::iter() {
                                        ui.selectable_value(
                                            &mut self.next_rank,
                                            rank,
                                            format!("{:?}", rank),
                                        );
                                    }
                                });
                        });
                    ui.advance_cursor_after_rect(ir.response.rect);
                    if ui.button("Add").clicked() {
                        let x = rand::thread_rng().gen_range(0..self.screen_width as i32) as f32;
                        let y = rand::thread_rng().gen_range(0..self.screen_height as i32) as f32;
                        let card = example::ConventionalCard {
                            suit: self.next_suit.clone(),
                            rank: self.next_rank.clone(),
                            pos: egui::Pos2::from((x, y)),
                        };
                        self.cards.push(card);
                        #[cfg(target_arch = "wasm32")]
                        log(format!("added card @ ({}|{})", x, y).as_str());
                    }
                }
            });
        });
    }
}
