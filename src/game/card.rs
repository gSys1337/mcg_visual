pub mod example;

#[cfg(target_arch = "wasm32")]
use crate::log;
use egui::{load, Sense};

pub trait Card {
    fn draw(&mut self, ui: &mut egui::Ui) -> egui::Response;
}

pub struct Backside {
    pub(crate) printed: bool,
}

impl Backside {
    pub fn new() -> Self {
        Self { printed: false }
    }
}

impl Card for Backside {
    fn draw(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let texture_res = ui.ctx().try_load_texture(
            format!("bytes://../../media/img_cards/card_back.png").as_str(),
            Default::default(),
            load::SizeHint::Size(100, 144),
        );
        match texture_res {
            Ok(poll) => match poll.texture_id() {
                None => ui.image(egui::include_image!(
                    "../../media/img_cards/placeholder.png"
                )),
                Some(t) => {
                    if !self.printed {
                        #[cfg(target_arch = "wasm32")]
                        log("Texture loaded!");
                        self.printed = true;
                    }
                    let texture = load::SizedTexture::new(t, egui::Vec2::new(100.0, 144.0));
                    let img = egui::Image::from_texture(texture)
                        .show_loading_spinner(true)
                        .sense(Sense::click_and_drag());
                    ui.add(img)
                }
            },
            Err(err) => {
                if !self.printed {
                    #[cfg(target_arch = "wasm32")]
                    log(format!("{:?}", err).as_str());
                    self.printed = true;
                }
                ui.image(egui::include_image!("../../media/img_cards/failed.png"))
            }
        }
    }
}
