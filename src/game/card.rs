// #[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
use crate::log;
use crate::openDirectoryPicker;
use egui::Sense;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures::js_sys::Array;
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[derive(Clone)]
pub struct DirectoryCardType {
    path: String,
    card_list: Vec<String>,
}

impl DirectoryCardType {
    pub fn new_from_selection(holder: Rc<RefCell<Option<DirectoryCardType>>>) {
        let type_rc = Rc::clone(&holder);
        spawn_local(async move {
            let x = JsFuture::from(openDirectoryPicker()).await;
            match x {
                Ok(x) => {
                    let mut card_type = Self {
                        path: String::new(),
                        card_list: Vec::new(),
                    };
                    let arr: Array = x.into();
                    for file in arr {
                        let thing: Array = Array::from(&file);
                        let t: Vec<String> = thing.iter().map(|x| x.as_string().unwrap()).collect();
                        card_type.card_list.push(t.get(0).unwrap().clone());
                    }
                    type_rc.borrow_mut().replace(card_type);
                }
                Err(_) => {}
            }
        });
    }
    pub fn all(&self) -> impl IntoIterator<Item = String> {
        self.card_list.clone()
    }
}

pub trait Card {
    // TODO replace img_path(...) with img(...)
    fn img_path(&self) -> String;
    fn pos(&self) -> egui::Pos2;
    fn set_pos(&mut self, pos: egui::Pos2);
    fn translate(&mut self, amt: egui::Vec2);
}

pub trait Drawable: egui::Widget + Sized {
    fn pos(&self) -> egui::Pos2;
    fn draw(
        self,
        ui: &mut egui::Ui,
        pos: Option<egui::Pos2>,
        sense: Option<Sense>,
        order: Option<egui::Order>,
        interactable: bool,
    ) -> egui::InnerResponse<egui::Response> {
        let mut area = egui::Area::new(ui.next_auto_id()).interactable(interactable);
        match pos {
            Some(pos) => area = area.current_pos(pos),
            None => area = area.current_pos(self.pos()),
        }
        sense.iter().for_each(|sense| {
            area = area.sense(*sense);
        });
        order.iter().for_each(|order| {
            area = area.order(*order);
        });
        // important to use ``&*card`` because rust gets it somehow wrong ¯\_(ツ)_/¯
        // i assume this is an "issue" with deref coercion because
        // Rust can include `*`s at compile-time but no `&`
        area.show(ui.ctx(), |ui| ui.add(self))
    }
}

impl Drawable for &dyn Card {
    fn pos(&self) -> egui::Pos2 {
        (*self).pos()
    }
}

// impl dyn Card {
//     pub fn draw(
//         &self,
//         ui: &mut egui::Ui,
//         pos: Option<egui::Pos2>,
//         sense: Option<Sense>,
//         order: Option<egui::Order>,
//     ) -> egui::InnerResponse<egui::Response> {
//         let mut area = egui::Area::new(ui.next_auto_id());
//         match pos {
//             Some(pos) => area = area.current_pos(pos),
//             None => area = area.current_pos(self.pos()),
//         }
//         sense.iter().for_each(|sense| {
//             area = area.sense(*sense);
//         });
//         order.iter().for_each(|order| {
//             area = area.order(*order);
//         });
//         // important to use ``&*card`` because rust gets it somehow wrong ¯\_(ツ)_/¯
//         // i assume this is an "issue" with deref coercion because
//         // Rust can include `*`s at compile-time but no `&`
//         area.show(ui.ctx(), |ui| ui.add(&*self))
//     }
// }

impl egui::Widget for &dyn Card {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let path = self.img_path();
        let img = egui::Image::new(path)
            .show_loading_spinner(true)
            .fit_to_original_size(1.0)
            .sense(Sense::click_and_drag());
        ui.add(img)
    }
}
