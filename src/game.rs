use eframe::emath;
use egui::{Context, Direction};
use rand::Rng;
use std::cell::RefCell;

// #[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
use crate::log;
pub mod card;
pub mod vfx;
use crate::example;
pub use crate::game::card::Card;
use crate::game::card::{DirectoryCardType, Drawable};
pub use vfx::Field;

pub struct State {
    players: usize,
    player_cards: Vec<Box<dyn Field>>,
}

impl Default for State {
    fn default() -> Self {
        let mut first: example::HandLayout = Default::default();
        for _ in 0..10 {
            first.add_card(Box::new(example::ConventionalCard::new_random()));
        }
        Self {
            players: 2,
            player_cards: vec![Box::new(first), Box::new(example::HandLayout::default())],
        }
    }
}

pub struct App {
    card_types: Rc<RefCell<Option<DirectoryCardType>>>,
    current_state: State,
    cards: Vec<Box<dyn Card>>,
    next_suit: usize,
    next_rank: usize,
    screen_width: f32,
    screen_height: f32,
    hand: example::HandLayout,
    stack: Box<dyn Field>,
}

impl App {
    #[allow(unused)]
    pub fn new(cc: &eframe::CreationContext) -> Self {
        crate::utils::set_panic_hook();
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut cards: Vec<Box<dyn Card>> = Vec::new();
        cards.push(Box::new(example::ConventionalCard::new_random()));
        let mut hand: example::HandLayout = Default::default();
        for _ in 0..10 {
            hand.add_card(Box::new(example::ConventionalCard::new_random()));
        }
        let mut stack: example::Stack = Default::default();
        for _ in 0..10 {
            stack.add_card(Box::new(example::ConventionalCard::new_random()));
        }
        stack.pos.x += 400.0;
        let stack: Box<dyn Field> = Box::new(stack);
        Self {
            card_types: Rc::new(RefCell::new(None)),
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

impl FromStr for Anchor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Menu" => Ok(Anchor::Menu),
            "Game" => Ok(Anchor::Game),
            "Settings" => Ok(Anchor::Settings),
            _ => Err("Provided &str doesn't look like any type of Anchor"),
        }
    }
}

use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::str::FromStr;

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = 30.0;
            // TODO move text size into settings
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
            // #[cfg(target_arch = "wasm32")]
            let window = _frame
                .info()
                .web_info
                .location
                .hash
                .strip_prefix('#')
                .and_then(|s| s.parse::<Anchor>().ok())
                .unwrap_or(Anchor::Menu);
            // #[cfg(target_arch = "wasm32")]
            /* TODO create window trait? in order to move this code to different files
                and more important: give the user of this library a way to customise
            */
            ui.with_layout(layout, |ui| {
                match window {
                    Anchor::Menu => {
                        let start = egui::Button::new("Start Game");
                        if ui.add(start).clicked() && self.card_types.borrow().is_some() {
                            log(format!(
                                "Card Type: {:?}",
                                self.card_types.borrow().as_ref().unwrap()
                            )
                            .as_str());
                            // #[cfg(target_arch = "wasm32")]
                            log("game started");
                            let delta = self.current_state.player_cards.len() as isize
                                - self.current_state.players as isize;
                            let f = if delta >= 0 {
                                |v: &mut Vec<Box<dyn Field>>| {
                                    v.pop();
                                }
                            } else {
                                |v: &mut Vec<Box<dyn Field>>| {
                                    v.push(Box::new(example::HandLayout::default()))
                                }
                            };
                            // #[cfg(target_arch = "wasm32")]
                            log(format!(
                                "len: {}\nplayers: {}\ndelta: {}",
                                self.current_state.player_cards.len(),
                                self.current_state.players,
                                delta
                            )
                            .as_str());
                            let x = 0..delta.abs();
                            x.for_each(|_| f(&mut self.current_state.player_cards));
                            self.current_state
                                .player_cards
                                .iter_mut()
                                .enumerate()
                                .for_each(|(i, hand)| {
                                    let pos = egui::pos2(70.0, 170.0 * (i + 1) as f32);
                                    hand.set_pos(pos);
                                });
                            ctx.open_url(egui::OpenUrl::same_tab(format!("#{:?}", Anchor::Game)));
                            if self.current_state.players >= 2 {
                                let mut field: example::HandLayout = Default::default();
                                let card_type = Rc::new(self.card_types.borrow().clone().unwrap());
                                for t in 0..10usize {
                                    field.add_card(Box::new(card::SimpleCard::new(
                                        t,
                                        Rc::clone(&card_type),
                                    )));
                                }
                                self.current_state.player_cards[1] = Box::new(field);
                            }
                        };
                        let settings = egui::Button::new("Settings");
                        if ui.add(settings).clicked() {
                            // #[cfg(target_arch = "wasm32")]
                            log("configuring textures");
                            ctx.open_url(egui::OpenUrl::same_tab(format!(
                                "#{:?}",
                                Anchor::Settings
                            )));
                        }
                        if ui.button("Open Directory").clicked() {
                            DirectoryCardType::new_from_selection(Rc::clone(&self.card_types));
                        }
                    }
                    Anchor::Game => {
                        let back = egui::Button::new("Back");
                        if ui.add(back).clicked() {
                            // #[cfg(target_arch = "wasm32")]
                            log("back to main menu");
                            ctx.open_url(egui::OpenUrl::same_tab(format!("#{:?}", Anchor::Menu)));
                        }
                        // ui.add(&mut self.hand);
                        // ui.advance_cursor_after_rect(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(100000.0, 1000.0)));
                        for player in self.current_state.player_cards.iter_mut() {
                            // let r = ui.add(&**player);
                            // ui.advance_cursor_after_rect(r.rect);
                            player.draw(ui, None, None, None, false);
                            // player.draw(ui, None, Some(egui::Sense::empty()), None, false);
                        }
                        // ui.add(&*self.stack);
                        self.stack
                            .draw(ui, None, Some(egui::Sense::empty()), None, true);
                        for card in self.cards.iter_mut() {
                            let sense = egui::Sense::click_and_drag();
                            // TODO investigate bug that all cards share the same egui::ID
                            let ir = card.draw(ui, None, Some(sense), None, true);
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
                            // #[cfg(target_arch = "wasm32")]
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
                            let x =
                                rand::thread_rng().gen_range(0..self.screen_width as i32) as f32;
                            let y =
                                rand::thread_rng().gen_range(0..self.screen_height as i32) as f32;
                            let card = example::ConventionalCard {
                                suit: example::Suit::from(self.next_suit),
                                rank: example::Rank::from(self.next_rank),
                                pos: egui::Pos2::from((x, y)),
                            };
                            self.cards.push(Box::new(card));
                            // #[cfg(target_arch = "wasm32")]
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
                }
            });
        });
    }
}
