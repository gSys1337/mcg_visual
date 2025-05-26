use crate::game::card::{CardConfig, CardEncoding, DirectoryCardType, SimpleCard};
use crate::game::field::{FieldWidget, SimpleField, SimpleFieldKind::Stack};
use crate::sprintln;
use eframe::Frame;
use egui::{vec2, Align, Context, Layout, UiBuilder};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub trait ScreenWidget {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, frame: &mut Frame);
}
impl ScreenWidget for MainMenu {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut rect = ui.max_rect();
            let width = rect.width() / 3.0;
            rect.set_left(width);
            rect.set_right(2.0 * width);
            ui.allocate_new_ui(
                UiBuilder::new()
                    .layout(Layout::top_down_justified(Align::Min))
                    .max_rect(rect),
                |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add_space(20.0);
                        if ui.button("Start").clicked() {
                            sprintln!("setup started");
                            *next_screen.borrow_mut() = String::from("game_setup");
                        };
                        ui.add_space(5.0);
                        if ui.button("Settings").clicked() {
                            sprintln!("settings opened");
                            *next_screen.borrow_mut() = String::from("settings");
                        };
                        ui.add_space(5.0);
                        if ui.button("Print Screen").clicked() {
                            sprintln!("{}", next_screen.borrow());
                        };
                    });
                },
            );
        });
    }
}
impl ScreenWidget for GameSetupScreen {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut rect = ui.max_rect();
            let width = rect.width() / 3.0;
            rect.set_left(width);
            rect.set_right(2.0 * width);
            ui.allocate_new_ui(
                UiBuilder::new()
                    .layout(Layout::top_down_justified(Align::Min))
                    .max_rect(rect),
                |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            ui.label("Selected Directory:");
                            match self.directory.borrow().as_ref() {
                                None => ui.label("None"),
                                Some(dir) => ui.label(&dir.path),
                            }
                        });
                        #[cfg(target_arch = "wasm32")]
                        ui.add_space(5.0);
                        if ui.button("Select Directory").clicked() {
                            #[cfg(target_arch = "wasm32")]
                            DirectoryCardType::new_from_selection(Rc::clone(&self.directory));
                        }
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.label("# Players");
                            let drag = egui::DragValue::new(&mut self.players);
                            ui.add(drag);
                            let dec = egui::Button::new("-").min_size(vec2(30.0, 0.0));
                            if ui.add(dec).clicked() && self.players > 1 {
                                self.players = self.players.saturating_sub(1);
                            }
                            let inc = egui::Button::new("+").min_size(vec2(30.0, 0.0));
                            if ui.add(inc).clicked() {
                                self.players = self.players.saturating_add(1);
                            }
                        });
                        ui.add_space(5.0);
                        if ui.button("Start Game").clicked() {
                            if let Some(game) = self.game_widget.upgrade() {
                                let config = self.generate_config();
                                if config.is_some() {
                                    game.borrow_mut().game_config = config;
                                    *next_screen.borrow_mut() = String::from("game");
                                }
                            }
                        }
                        ui.add_space(5.0);
                        if ui.button("Back").clicked() {
                            *next_screen.borrow_mut() = String::from("main");
                        }
                    });
                },
            );
        });
    }
}
impl ScreenWidget for Game<DirectoryCardType> {
    fn update(&mut self, next_screen: Rc<RefCell<String>>, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut rect = ui.max_rect();
            let width = rect.width() / 3.0;
            rect.set_left(width);
            rect.set_right(2.0 * width);
            ui.allocate_new_ui(
                UiBuilder::new()
                    .layout(Layout::top_down_justified(Align::Min))
                    .max_rect(rect),
                |ui| {
                    ui.add_space(20.0);
                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Exit").clicked() {
                            *next_screen.borrow_mut() = String::from("main");
                        }
                    });
                    if self.game_config.is_none() {
                        return;
                    }
                    ui.add_space(5.0);
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("1. Player:");
                            egui::ComboBox::from_id_salt("Display Player 0").show_index(
                                ui,
                                &mut self.player0_idx,
                                self.game_config.as_mut().unwrap().players.len(),
                                |i| i.to_string(),
                            );
                            ui.add_space(
                                3.0 * width - 2.0 * ui.cursor().left()
                                    + ui.spacing().item_spacing.x,
                            );
                            ui.label("2. Player:");
                            egui::ComboBox::from_id_salt("Display Player 1").show_index(
                                ui,
                                &mut self.player1_idx,
                                self.game_config.as_mut().unwrap().players.len(),
                                |i| i.to_string(),
                            );
                        });
                    });
                    ui.add_space(5.0);
                    let cfg = self.game_config.as_mut().unwrap();
                    ui.add_space(5.0);
                    ui.label("Stack");
                    let stack = &cfg.stack;
                    if let Some(_payload) =
                        ui.add(stack.draw()).dnd_release_payload::<DNDSelector>()
                    {
                        self.drop = Some(DNDSelector::Stack)
                    }
                    match stack.get_payload() {
                        (_, Some(_idx)) => self.drop = Some(DNDSelector::Stack),
                        (Some(_idx), _) => {
                            if self.drag.is_none() {
                                self.drag = Some(DNDSelector::Stack)
                            }
                        }
                        (None, None) => {}
                    }
                    let (name_0, field_0) = &cfg.players[self.player0_idx];
                    ui.add_space(5.0);
                    ui.label(name_0);
                    if let Some(_payload) =
                        ui.add(field_0.draw()).dnd_release_payload::<DNDSelector>()
                    {
                        self.drop = Some(DNDSelector::Player(self.player0_idx, field_0.cards.len()))
                    }
                    match field_0.get_payload() {
                        (_, Some(idx)) => {
                            self.drop = Some(DNDSelector::Player(self.player0_idx, idx))
                        }
                        (Some(idx), _) => {
                            if self.drag.is_none() {
                                self.drag = Some(DNDSelector::Player(self.player0_idx, idx))
                            }
                        }
                        (None, None) => {}
                    }
                    let (name_1, field_1) = &cfg.players[self.player1_idx];
                    ui.add_space(5.0);
                    ui.label(name_1);
                    if let Some(_payload) =
                        ui.add(field_1.draw()).dnd_release_payload::<DNDSelector>()
                    {
                        self.drop = Some(DNDSelector::Player(self.player1_idx, field_1.cards.len()))
                    }
                    match field_1.get_payload() {
                        (_, Some(idx)) => {
                            self.drop = Some(DNDSelector::Player(self.player1_idx, idx))
                        }
                        (Some(idx), _) => {
                            if self.drag.is_none() {
                                self.drag = Some(DNDSelector::Player(self.player1_idx, idx))
                            }
                        }
                        (None, None) => {}
                    }
                    if let (Some(source), Some(destination)) = (self.drag, self.drop) {
                        cfg.move_card::<SimpleCard>(source, destination);
                        self.drag = None;
                        self.drop = None;
                    }
                },
            );
        });
    }
}

pub struct MainMenu {}
impl MainMenu {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GameSetupScreen<C: CardConfig = DirectoryCardType, G = Game<C>> {
    directory: Rc<RefCell<Option<C>>>,
    players: usize,
    pub(crate) game_widget: Weak<RefCell<G>>,
}
impl<C: CardConfig + Clone, G> GameSetupScreen<C, G> {
    pub fn new(game_widget: Weak<RefCell<G>>) -> Self {
        let directory = Rc::new(RefCell::new(None));
        let players = 2;
        Self {
            directory,
            players,
            game_widget,
        }
    }
    fn generate_config(&self) -> Option<GameState<C>> {
        let directory = Rc::new(self.directory.borrow().clone()?);
        let mut players: Vec<(String, SimpleField<SimpleCard, C>)> = (0..self.players)
            .map(|i| {
                (
                    format!("{i}"),
                    SimpleField::new(Rc::clone(&directory))
                        .max_cards(4)
                        .selectable(true)
                        .max_card_size(vec2(100.0, 150.0)),
                )
            })
            .collect();
        let mut stack = SimpleField::new(Rc::clone(&directory))
            .kind(Stack)
            .max_card_size(vec2(100.0, 150.0));
        for i in 0..directory.T() {
            let card = SimpleCard::Open(i);
            stack.push(card);
            players[i % self.players].1.push(SimpleCard::Open(i));
        }
        Some(GameState { players, stack })
    }
}

/// Struct for a game with one stack and arbitrary players
pub struct Game<C: CardConfig> {
    pub(crate) game_config: Option<GameState<C>>,
    player0_idx: usize,
    player1_idx: usize,
    drag: Option<DNDSelector>,
    drop: Option<DNDSelector>,
}
impl<C: CardConfig> Game<C> {
    pub fn new() -> Self {
        Self {
            game_config: None,
            player0_idx: 0,
            player1_idx: 1,
            drag: None,
            drop: None,
        }
    }
}
impl<C: CardConfig> Default for Game<C> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GameState<C: CardConfig> {
    players: Vec<(String, SimpleField<SimpleCard, C>)>,
    stack: SimpleField<SimpleCard, C>,
}
impl<C: CardConfig> GameState<C> {
    pub fn move_card<E: CardEncoding>(&mut self, src: DNDSelector, dst: DNDSelector) {
        if src == dst {
            return;
        }
        let card = match src {
            DNDSelector::Player(p_idx, c_idx) => self.players[p_idx].1.remove(c_idx),
            DNDSelector::Stack => self.stack.cards.pop().unwrap(),
            DNDSelector::Index(_) => return,
        };
        match dst {
            DNDSelector::Player(p_idx, c_idx) => self.players[p_idx].1.insert(c_idx, card),
            DNDSelector::Stack => self.stack.cards.push(card),
            #[allow(clippy::needless_return)]
            DNDSelector::Index(_) => return,
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DNDSelector {
    Player(usize, usize),
    Stack,
    Index(usize),
}
