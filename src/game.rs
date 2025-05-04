use egui::Context;
use std::cell::RefCell;
use std::collections::{hash_map, HashMap};
use std::rc::Rc;
pub mod card;
pub mod screen;
use screen::{MainMenu, ScreenWidget};

pub struct App {
    screens: HashMap<String, Rc<RefCell<dyn ScreenWidget>>>,
    default_screen: Rc<RefCell<dyn ScreenWidget>>,
    current_screen: Rc<RefCell<String>>,
}

impl Default for App {
    fn default() -> Self {
        Self::new(None)
    }
}

impl App {
    #[allow(unused)]
    pub fn new(main_screen: Option<Rc<RefCell<dyn ScreenWidget>>>) -> Self {
        let default_screen = main_screen.unwrap_or_else(|| Rc::new(RefCell::new(MainMenu::new())));
        let current_screen = Rc::new(RefCell::new(String::from("main")));
        let mut screens = HashMap::new();
        screens.insert(String::from("main"), Rc::clone(&default_screen));
        Self {
            screens,
            default_screen,
            current_screen,
        }
    }
    #[allow(clippy::result_unit_err)]
    pub fn register_screen(
        &mut self,
        name: String,
        screen: Rc<RefCell<dyn ScreenWidget>>,
    ) -> Result<(), ()> {
        if let hash_map::Entry::Vacant(e) = self.screens.entry(name) {
            e.insert(screen);
            Ok(())
        } else {
            Err(())
        }
    }
}

// use egui::FontFamily::Proportional;
// use egui::FontId;
// use egui::TextStyle::*;
// use std::collections::BTreeMap;

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let current_screen = match self.screens.get_mut(&self.current_screen.borrow().clone()) {
            Some(screen) => Rc::clone(screen),
            None => Rc::clone(&self.default_screen),
        };
        let next_screen = Rc::clone(&self.current_screen);
        current_screen.borrow_mut().update(next_screen, ctx, frame);
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     let size = 30.0;
        //     // TODO move text size into settings
        //     let text_styles: BTreeMap<_, _> = [
        //         (Heading, FontId::new(size, Proportional)),
        //         (Name("Heading2".into()), FontId::new(size, Proportional)),
        //         (Name("Context".into()), FontId::new(size, Proportional)),
        //         (Body, FontId::new(size, Proportional)),
        //         (Monospace, FontId::new(size, Proportional)),
        //         (Button, FontId::new(size, Proportional)),
        //         (Small, FontId::new(size, Proportional)),
        //     ]
        //     .into();
        //
        //     // Mutate global styles with new text styles
        //     ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
        //
        //     let layout = egui::Layout::from_main_dir_and_cross_align(
        //         Direction::TopDown,
        //         emath::Align::Center,
        //     );
        //     // #[cfg(target_arch = "wasm32")]
        //     let window = frame
        //         .info()
        //         .web_info
        //         .location
        //         .hash
        //         .strip_prefix('#')
        //         .and_then(|s| s.parse::<Anchor>().ok())
        //         .unwrap_or(Anchor::Menu);
        //     // #[cfg(target_arch = "wasm32")]
        //     /* TODO create window trait? in order to move this code to different files
        //         and more important: give the user of this library a way to customise
        //     */
        //     ui.with_layout(layout, |ui| {
        //         match window {
        //             Anchor::Menu => {
        //                 let start = egui::Button::new("Start Game");
        //                 if ui.add(start).clicked() && self.card_types.borrow().is_some() {
        //                     log(format!(
        //                         "Card Type: {:?}",
        //                         self.card_types.borrow().as_ref().unwrap()
        //                     )
        //                     .as_str());
        //                     // #[cfg(target_arch = "wasm32")]
        //                     log("game started");
        //                     let delta = self.current_state.player_cards.len() as isize
        //                         - self.current_state.players as isize;
        //                     let f = if delta >= 0 {
        //                         |v: &mut Vec<_>, _idx| {
        //                             v.pop();
        //                         }
        //                     } else {
        //                         |v: &mut Vec<_>, idx| {
        //                             let pos = egui::pos2(70.0, 170.0 * (idx + 1) as f32);
        //                             v.push(SimpleField::new().pos(pos))
        //                         }
        //                     };
        //                     // #[cfg(target_arch = "wasm32")]
        //                     log(format!(
        //                         "len: {}\nplayers: {}\ndelta: {}",
        //                         self.current_state.player_cards.len(),
        //                         self.current_state.players,
        //                         delta
        //                     )
        //                     .as_str());
        //                     let len = self.current_state.player_cards.len();
        //                     let x = 0..delta.unsigned_abs();
        //                     x.for_each(|i| f(&mut self.current_state.player_cards, i + len));
        //                     ctx.open_url(egui::OpenUrl::same_tab(format!("#{:?}", Anchor::Game)));
        //                 };
        //                 let settings = egui::Button::new("Settings");
        //                 if ui.add(settings).clicked() {
        //                     // #[cfg(target_arch = "wasm32")]
        //                     log("configuring textures");
        //                     ctx.open_url(egui::OpenUrl::same_tab(format!(
        //                         "#{:?}",
        //                         Anchor::Settings
        //                     )));
        //                 }
        //                 if ui.button("Open Directory").clicked() {
        //                     DirectoryCardType::new_from_selection(Rc::clone(&self.card_types));
        //                 }
        //             }
        //             Anchor::Game => {
        //                 let back = egui::Button::new("Back");
        //                 if ui.add(back).clicked() {
        //                     // #[cfg(target_arch = "wasm32")]
        //                     log("back to main menu");
        //                     ctx.open_url(egui::OpenUrl::same_tab(format!("#{:?}", Anchor::Menu)));
        //                 }
        //                 // ui.add(&mut self.hand);
        //                 // ui.advance_cursor_after_rect(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(100000.0, 1000.0)));
        //                 for player in &self.current_state.player_cards {
        //                     // let r = ui.add(&**player);
        //                     // ui.advance_cursor_after_rect(r.rect);
        //                     ui.add(player.draw());
        //                     // player.draw(ui, None, Some(egui::Sense::empty()), None, false);
        //                 }
        //             }
        //             Anchor::Settings => {
        //                 let back = egui::Button::new("Back");
        //                 self.screen_width = ctx.available_rect().width();
        //                 self.screen_height = ctx.available_rect().height();
        //                 if ui.add(back).clicked() {
        //                     // #[cfg(target_arch = "wasm32")]
        //                     log("back to main menu");
        //                     ctx.open_url(egui::OpenUrl::same_tab(format!("#{:?}", Anchor::Menu)));
        //                 }
        //                 if self.card_types.borrow().is_some() {
        //                     let card_type = self.card_types.borrow().as_ref().unwrap().clone();
        //                     egui::ComboBox::from_label("Card").show_index(
        //                         ui,
        //                         &mut self.next_card,
        //                         card_type.all_images().len(),
        //                         |idx| card_type.all_images().nth(idx).unwrap().to_string(),
        //                     );
        //                     egui::ComboBox::from_label("Player").show_index(
        //                         ui,
        //                         &mut self.next_player,
        //                         self.current_state.players,
        //                         |idx| (idx + 1).to_string(),
        //                     );
        //                     if ui.button("Add").clicked() {
        //                         let card_type =
        //                             Rc::new(self.card_types.borrow().as_ref().unwrap().clone());
        //                         self.current_state.player_cards[self.next_player]
        //                             .push(SimpleCard::new(self.next_card, card_type));
        //                     }
        //                 }
        //                 let player_area = egui::Area::new(egui::Id::new("player"))
        //                     .sense(egui::Sense::click())
        //                     .current_pos(ui.next_widget_position());
        //                 let ir = player_area.show(ctx, |ui| {
        //                     ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        //                         ui.label("# Players");
        //                         let drag = egui::DragValue::new(&mut self.current_state.players);
        //                         ui.add(drag);
        //                         let dec = egui::Button::new("-").min_size(egui::vec2(50.0, 0.0));
        //                         if ui.add(dec).clicked() {
        //                             self.current_state.players =
        //                                 self.current_state.players.saturating_sub(1);
        //                         }
        //                         let inc = egui::Button::new("+").min_size(egui::vec2(50.0, 0.0));
        //                         if ui.add(inc).clicked() {
        //                             self.current_state.players =
        //                                 self.current_state.players.saturating_add(1);
        //                         }
        //                     })
        //                 });
        //                 ui.advance_cursor_after_rect(ir.response.rect);
        //             }
        //         }
        //     });
        // });
    }
}
