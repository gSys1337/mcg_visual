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

pub struct State {
    players: usize,
    player_cards: Vec<vfx::HandLayout>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            players: 2,
            player_cards: vec![Default::default(), Default::default()],
        }
    }
}

pub struct App {
    current_state: State,
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
            current_state: Default::default(),
            cards,
            next_suit: Default::default(),
            next_rank: Default::default(),
            screen_width: 0.0,
            screen_height: 0.0,
            hand,
            stack,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Anchor {
    Menu,
    Game,
    Settings,
}

impl Anchor {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Menu" => Some(Anchor::Menu),
            "Game" => Some(Anchor::Game),
            "Settings" => Some(Anchor::Settings),
            _ => None,
        }
    }
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
            #[cfg(target_arch = "wasm32")]
            let window = _frame
                .info()
                .web_info
                .location
                .hash
                .strip_prefix('#')
                .and_then(|s| Anchor::from_str(s))
                .unwrap_or(Anchor::Menu);
            #[cfg(target_arch = "wasm32")]
            ui.with_layout(layout, |ui| match window {
                Anchor::Menu => {
                    let start = egui::Button::new("Start Game");
                    if ui.add(start).clicked() {
                        #[cfg(target_arch = "wasm32")]
                        log("game started");
                        let delta = self.current_state.player_cards.len() as isize
                            - self.current_state.players as isize;
                        let f = if delta >= 0 {
                            |v: &mut Vec<vfx::HandLayout>| {
                                v.pop();
                            }
                        } else {
                            |v: &mut Vec<vfx::HandLayout>| v.push(vfx::HandLayout::default())
                        };
                        #[cfg(target_arch = "wasm32")]
                        log(format!(
                            "len: {}\nplayers: {}\ndelta: {}",
                            self.current_state.player_cards.len(),
                            self.current_state.players,
                            delta
                        )
                        .as_str());
                        let x = 0..delta.abs();
                        x.for_each(|_| f(&mut self.current_state.player_cards));
                        ctx.open_url(egui::OpenUrl::same_tab(format!("#{:?}", Anchor::Game)));
                    };
                    let settings = egui::Button::new("Settings");
                    if ui.add(settings).clicked() {
                        #[cfg(target_arch = "wasm32")]
                        log("configuring textures");
                        ctx.open_url(egui::OpenUrl::same_tab(format!("#{:?}", Anchor::Settings)));
                    }
                }
                Anchor::Game => {
                    let back = egui::Button::new("Back");
                    if ui.add(back).clicked() {
                        #[cfg(target_arch = "wasm32")]
                        log("back to main menu");
                        ctx.open_url(egui::OpenUrl::same_tab(format!("#{:?}", Anchor::Menu)));
                    }
                    // ui.add(&mut self.hand);
                    // ui.add(&mut self.stack);
                    // ui.advance_cursor_after_rect(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(100000.0, 1000.0)));
                    for player in self.current_state.player_cards.iter_mut() {
                        let r = ui.add(player);
                        ui.advance_cursor_after_rect(r.rect);
                    }
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
                        ctx.open_url(egui::OpenUrl::same_tab(format!("#{:?}", Anchor::Menu)));
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
                    let player_area = egui::Area::new(egui::Id::new("player"))
                        .sense(egui::Sense::click())
                        .current_pos(ui.next_widget_position());
                    let ir = player_area.show(ctx, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label("# Players");
                            let drag = egui::DragValue::new(&mut self.current_state.players);
                            ui.add(drag);
                            let dec = egui::Button::new("-").min_size(egui::vec2(50.0, 0.0));
                            if ui.add(dec).clicked() {
                                self.current_state.players =
                                    self.current_state.players.saturating_sub(1);
                            }
                            let inc = egui::Button::new("+").min_size(egui::vec2(50.0, 0.0));
                            if ui.add(inc).clicked() {
                                self.current_state.players =
                                    self.current_state.players.saturating_add(1);
                            }
                        })
                    });
                    ui.advance_cursor_after_rect(ir.response.rect);
                }
            });
        });
    }
}
