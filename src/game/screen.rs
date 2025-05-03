use crate::game::card::SimpleFieldKind::Stack;
use crate::game::card::{DirectoryCardType, FieldWidget, SimpleCard, SimpleField};
use crate::log;
use eframe::Frame;
use egui::Context;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub trait ScreenWidget {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, frame: &mut Frame);
}
pub struct MainMenu {}
impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl MainMenu {
    pub fn new() -> Self {
        Self {}
    }
}
impl ScreenWidget for MainMenu {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Start").clicked() {
                log("setup started");
                *next_screen.borrow_mut() = String::from("game_setup");
            };
            if ui.button("Settings").clicked() {
                log("settings opened");
                *next_screen.borrow_mut() = String::from("settings");
            };
            if ui.button("Print Screen").clicked() {
                log(&RefCell::borrow(&next_screen));
            };
        });
    }
}
pub struct GameSetupScreen {
    directory: Rc<RefCell<Option<DirectoryCardType>>>,
    players: usize,
    pub(crate) game_widget: Weak<RefCell<Game>>,
}
impl GameSetupScreen {
    pub fn new(game_widget: Weak<RefCell<Game>>) -> Self {
        let directory = Rc::new(RefCell::new(None));
        let players = 1;
        Self {
            directory,
            players,
            game_widget,
        }
    }
    fn generate_config(&self) -> Option<GameConfig> {
        let directory = Rc::new(self.directory.borrow().clone()?);
        let mut players: Vec<(String, SimpleField<SimpleCard<DirectoryCardType>>)> = (0..self.players)
            .map(|i| (format!("{i}"), SimpleField::new().max_cards(6)))
            .collect();
        let mut stack = SimpleField::new().kind(Stack);
        for i in 0..directory.img_names.len() {
            let card = SimpleCard::new(i, Rc::clone(&directory));
            stack.push(card);
            players[i%self.players].1.push(SimpleCard::new(i, Rc::clone(&directory)));
        }
        Some(GameConfig {
            directory,
            players,
            stack,
        })
    }
}
impl ScreenWidget for GameSetupScreen {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Selected Directory:");
                match self.directory.borrow().as_ref() {
                    None => ui.label("None"),
                    Some(dir) => ui.label(&dir.path),
                }
            });
            if ui.button("Select Directory").clicked() {
                DirectoryCardType::new_from_selection(Rc::clone(&self.directory));
            }
            ui.horizontal(|ui| {
                ui.label("# Players");
                let drag = egui::DragValue::new(&mut self.players);
                ui.add(drag);
                let dec = egui::Button::new("-").min_size(egui::vec2(30.0, 0.0));
                if ui.add(dec).clicked() && self.players > 1 {
                    self.players = self.players.saturating_sub(1);
                }
                let inc = egui::Button::new("+").min_size(egui::vec2(30.0, 0.0));
                if ui.add(inc).clicked() {
                    self.players = self.players.saturating_add(1);
                }
            });
            if ui.button("Start Game").clicked() {
                if let Some(game) = self.game_widget.upgrade() {
                    let config = self.generate_config();
                    if config.is_some() {
                        game.borrow_mut().game_config = config;
                        *next_screen.borrow_mut() = String::from("game");
                    }
                }
            }
            if ui.button("Back").clicked() {
                log("back to main menu");
                *next_screen.borrow_mut() = String::from("main");
            }
        });
    }
}
pub struct Game {
    pub(crate) game_config: Option<GameConfig>,
    image_idx: usize,
    player_idx: usize,
}
impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
impl Game {
    pub fn new() -> Self {
        Self {
            game_config: None,
            image_idx: 0,
            player_idx: 0,
        }
    }
}
impl ScreenWidget for Game {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Exit").clicked() {
                log("back to main menu");
                *next_screen.borrow_mut() = String::from("main");
            }
            ui.horizontal(|ui| {
                ui.label("Image Directory:");
                ui.label(&self.game_config.as_ref().unwrap().directory.path);
            });
            let images = self
                .game_config
                .as_ref()
                .unwrap()
                .directory
                .img_names
                .clone();
            ui.horizontal(|ui| {
                ui.label("Images:");
                egui::ComboBox::from_id_salt("Image Name preview").show_index(
                    ui,
                    &mut self.image_idx,
                    self.game_config.as_ref().unwrap().directory.img_names.len(),
                    |i| &images[i],
                );
            });
            let player_names: Vec<String> = self
                .game_config
                .as_ref()
                .unwrap()
                .players
                .iter()
                .clone()
                .map(|e| e.0.clone())
                .collect();
            ui.horizontal(|ui| {
                ui.label("Player:");
                egui::ComboBox::from_id_salt("Display Player Fields").show_index(
                    ui,
                    &mut self.player_idx,
                    self.game_config.as_ref().unwrap().players.len(),
                    |i| &player_names[i],
                );
            });
            ui.horizontal(|ui| {
                ui.add(self.game_config.as_ref().unwrap().stack.draw());
                ui.add(self.game_config.as_ref().unwrap().players[self.player_idx].1.draw());
            });
            if ui.button("Log something").clicked() {
                log(format!("{}", self.game_config.as_ref().unwrap().stack.size).as_str());
            }
        });
    }
}
pub struct GameConfig {
    directory: Rc<DirectoryCardType>,
    players: Vec<(String, SimpleField<SimpleCard<DirectoryCardType>>)>,
    stack: SimpleField<SimpleCard<DirectoryCardType>>,
}
