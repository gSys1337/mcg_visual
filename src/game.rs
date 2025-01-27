mod card;

#[cfg(target_arch = "wasm32")]
use crate::log;
use eframe::emath::Align;
use eframe::Frame;
use egui::load::{BytesLoadResult, BytesLoader, TexturePoll};
use egui::{Context, Direction, TextBuffer};

pub struct App {
    cards_path: Option<String>,
    // file_dialog: egui_file_dialog::FileDialog,
    current_state: Anchor,
    // byte_loader: Arc<egui::load::DefaultBytesLoader>,
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
                    let y = egui::load::DefaultBytesLoader::default()
                        .load(ctx, "file://../media/img_cards/1_heart.png");
                    #[cfg(target_arch = "wasm32")]
                    match y {
                        Ok(y1) => {
                            //log(format!("Y: {:?}", y1).as_str());
                        }
                        Err(y2) => {
                            log(format!("Y: {:?}", y2).as_str());
                        }
                    }
                    let x = ctx.try_load_texture(
                        "file://../media/img_cards/1_heart.png",
                        Default::default(),
                        Default::default(),
                    );
                    match x {
                        Ok(poll) => match poll {
                            TexturePoll::Pending { .. } => {}
                            TexturePoll::Ready { texture } => {
                                ui.add(egui::Image::from_texture(texture));
                            }
                        },
                        Err(e) => {
                            #[cfg(target_arch = "wasm32")]
                            log(format!("Error: {:?}", e).as_str());
                        }
                    }
                }
                Anchor::Settings => {
                    let back = egui::Button::new("Back");
                    if ui.add(back).clicked() {
                        #[cfg(target_arch = "wasm32")]
                        log("back to main menu");
                        self.current_state = Anchor::Menu;
                    }
                }
            });
        });
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        // let byte_loader = Arc::new(egui::load::DefaultBytesLoader::default());
        // cc.egui_ctx.add_bytes_loader(byte_loader.clone());
        crate::utils::set_panic_hook();
        #[cfg(target_arch = "wasm32")]
        log("New App created.");
        Self {
            cards_path: None,
            current_state: Anchor::Menu,
            // file_dialog: egui_file_dialog::FileDialog::new(),
            // byte_loader,
        }
    }
}
