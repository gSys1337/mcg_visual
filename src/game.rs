use eframe::emath::Align;
use eframe::Frame;
use egui::{Context, Direction, Ui};
use crate::log;

pub struct App {
    cards_texture: Option<egui::TextureId>,
    current_state: Anchor,
}

enum Anchor {
    Menu,
    Game,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = egui::Layout::from_main_dir_and_cross_align(Direction::TopDown, Align::Center);
            ui.with_layout(layout, |ui| {
                let start: egui::Button = egui::Button::new("Start Game");
                ui.add(start);
            });
        });
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        crate::utils::set_panic_hook();
        log("New App created.");
        Self { cards_texture: None, current_state: Anchor::Menu }
    }
}

trait View {
    fn draw(&mut self, ui: &mut Ui);
}

struct Menu {}

impl Menu {
    fn new() -> Self {
        Self {}
    }
}

impl View for Menu {
    fn draw(&mut self, ui: &mut Ui) {
        if ui.button("Start Game").clicked() {

        }
    }
}
