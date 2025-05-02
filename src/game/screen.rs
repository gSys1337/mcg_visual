use std::cell::RefCell;
use std::rc::Rc;
use egui::{Context, TextBuffer};
use crate::log;

pub trait ScreenWidget {
    fn update(&mut self, screen: Rc<RefCell<&str>>, ctx: &Context, _frame: &mut eframe::Frame);
}
pub struct MainMenu {}
impl ScreenWidget for MainMenu {
    fn update(&mut self, screen: Rc<RefCell<&str>>, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Start").clicked() {
                log("game started");
                *screen.borrow_mut() = "game";
            };
            if ui.button("Settings").clicked() {
                log("settings opened");
                *screen.borrow_mut() = "settings";
            };
            if ui.button("Print Screen").clicked() {
                log(*RefCell::borrow(&*screen));
            };
        });
    }
}
pub struct Settings {}
pub struct Game {
    
}
