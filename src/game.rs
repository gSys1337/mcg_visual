mod card;

#[cfg(target_arch = "wasm32")]
use crate::log;
use eframe::emath::Align;
use eframe::Frame;
use egui::{Context, Direction};
use std::ops::Add;

pub struct App {
    card_sources: Vec<Vec<egui::ImageSource<'static>>>,
    current_state: Anchor,
    card: card::ConventionalCard,
}

enum Anchor {
    Menu,
    Game,
    Settings,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                    egui::Area::new(egui::Id::from("single_card"))
                        .sense(egui::Sense::click_and_drag())
                        .current_pos(self.card.pos.unwrap_or(egui::Pos2::new(100.0, 100.0)))
                        .show(&ctx, |ui| {
                            let (idx_suit, idx_rank) = self.card.get_source_index();
                            let card_img =
                                egui::Image::new(self.card_sources[idx_suit][idx_rank].clone())
                                    .sense(egui::Sense::click_and_drag())
                                    .fit_to_original_size(1.0);
                            let resp = ui.add(card_img);
                            if resp.is_pointer_button_down_on() {
                                let new = self
                                    .card
                                    .pos
                                    .unwrap_or(egui::Pos2::new(100.0, 100.0))
                                    .add(resp.drag_delta());
                                self.card.pos = Some(new);
                            }
                        });
                }
                Anchor::Settings => {
                    let back = egui::Button::new("Back");
                    if ui.add(back).clicked() {
                        #[cfg(target_arch = "wasm32")]
                        log("back to main menu");
                        self.current_state = Anchor::Menu;
                    }
                    egui::ComboBox::from_label("Suit")
                        .selected_text(format!("{:?}", self.card.suit))
                        .show_ui(ui, |ui| {
                            for suit in card::Suit::all_vec().iter() {
                                ui.selectable_value(
                                    &mut self.card.suit,
                                    *suit,
                                    format!("{:?}", suit),
                                );
                            }
                        });
                }
            });
        });
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        crate::utils::set_panic_hook();
        #[cfg(target_arch = "wasm32")]
        log("New App created.");
        Self {
            current_state: Anchor::Menu,
            card_sources: card::ConventionalCard::load_image_sources(),
            card: Default::default(),
        }
    }
}
