use crate::game::card::SimpleFieldKind::Stack;
use crate::game::card::{DirectoryCardType, SimpleCard, SimpleField};
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
        let players = 0;
        Self {
            directory,
            players,
            game_widget,
        }
    }
    /// Panics if no directory is selected!
    fn generate_config(&self) -> GameConfig {
        let directory = self
            .directory
            .borrow()
            .clone()
            .expect("Expecting only to be called when a directory has been selected");
        let players = (0..self.players)
            .map(|i| (format!("{i}"), SimpleField::new()))
            .collect();
        let stack = SimpleField::new().kind(Stack);
        GameConfig {
            directory,
            players,
            stack,
        }
    }
}
impl ScreenWidget for GameSetupScreen {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Selected Directory:");
                match self.directory.borrow().as_ref() {
                    None => ui.label("None"),
                    Some(dir) => ui.label(&dir.path),
                }
            });
            if ui.button("Select Directory").clicked() {
                DirectoryCardType::new_from_selection(Rc::clone(&self.directory));
            }
            ui.vertical(|ui| {
                ui.label("# Players");
                let drag = egui::DragValue::new(&mut self.players);
                ui.add(drag);
                let dec = egui::Button::new("-").min_size(egui::vec2(50.0, 50.0));
                if ui.add(dec).clicked() {
                    self.players = self.players.saturating_sub(1);
                }
                let inc = egui::Button::new("+").min_size(egui::vec2(50.0, 50.0));
                if ui.add(inc).clicked() {
                    self.players = self.players.saturating_add(1);
                }
            });
            if ui.button("Start Game").clicked() {
                if let Some(game) = self.game_widget.upgrade() {
                    game.borrow_mut().game_config = Some(self.generate_config());
                    *next_screen.borrow_mut() = String::from("game");
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
}
impl Game {
    pub fn new() -> Self {
        Self { game_config: None }
    }
}
impl ScreenWidget for Game {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Exit").clicked() {
                log("back to main menu");
                *next_screen.borrow_mut() = String::from("main");
            }
        });
    }
}
pub struct GameConfig {
    directory: DirectoryCardType,
    players: Vec<(String, SimpleField<SimpleCard<DirectoryCardType>>)>,
    stack: SimpleField<SimpleCard<DirectoryCardType>>,
}
