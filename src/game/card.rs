// #[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
use crate::log;
use crate::openDirectoryPicker;
use egui::frame;
use egui::ImageSource::Uri;
use std::borrow::Cow;
use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use std::slice::Iter;
use wasm_bindgen_futures::js_sys::Array;
use wasm_bindgen_futures::{spawn_local, JsFuture};

pub trait CardWidget {
    fn img(&self) -> egui::Image;
    // Is draw_at(...) needed when egui::Image::paint_at(...) exists?
    fn draw_at(&self, ui: &mut egui::Ui, pos: egui::Pos2) -> egui::InnerResponse<egui::Response> {
        let area = egui::Area::new(ui.next_auto_id()).current_pos(pos);
        area.show(ui.ctx(), |ui| ui.add(self.img()))
    }
}

pub struct SimpleCard<T> {
    images: Rc<T>,
    card_type: usize,
}

impl SimpleCard<DirectoryCardType> {
    pub fn new(card_type: usize, images: Rc<DirectoryCardType>) -> SimpleCard<DirectoryCardType> {
        assert!(card_type < images.T);
        Self { images, card_type }
    }
}

impl CardWidget for SimpleCard<DirectoryCardType> {
    fn img(&self) -> egui::Image {
        let path = format!(
            "http://127.0.0.1:8080/media/{folder}/{card}",
            folder = self.images.path,
            card = self.images.img_names[self.card_type]
        );
        egui::Image::new(path)
            .show_loading_spinner(true)
            .maintain_aspect_ratio(true)
    }
}

#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct DirectoryCardType {
    path: String,
    img_names: Vec<String>,
    T: usize,
    size: RefCell<Option<egui::Vec2>>,
}

impl DirectoryCardType {
    /// It's assumed the image URL is inside servers /media directory and the
    /// type order corresponds to the lexicographical.
    ///
    /// For real file upload you need to extend the simple python http server to accept uploads.
    /// Does pythons simple https server already accept POST requests?
    pub fn new_from_selection(holder: Rc<RefCell<Option<DirectoryCardType>>>) {
        let type_rc = Rc::clone(&holder);
        spawn_local(async move {
            let response = JsFuture::from(openDirectoryPicker()).await;
            if let Ok(file_info_array) = response {
                let mut card_type = Self {
                    path: String::new(),
                    img_names: Vec::new(),
                    T: 0usize,
                    // TODO get size from images
                    size: RefCell::new(None),
                };
                let mut path_set = false;
                let file_info_array: Array = file_info_array.into();
                for file_info in file_info_array {
                    let file_info: Array = Array::from(&file_info);
                    let file_info: Vec<String> = file_info
                        .iter()
                        .map(|x| x.as_string().unwrap().clone())
                        .collect();
                    let file_name = file_info.first().expect("Every file has a name!").clone();
                    let path = file_info.get(1).expect("Every file has a path!").clone();
                    let file_type = file_info.get(2);
                    if !path_set {
                        let path = path
                            .strip_suffix(format!("/{file_name}").as_str())
                            .unwrap()
                            .to_string();
                        card_type.path = path;
                        path_set = true;
                    }
                    if let Some(file_type) = file_type {
                        if file_type.starts_with("image") {
                            card_type.img_names.push(file_name);
                        }
                    }
                }
                card_type.img_names.sort();
                card_type.T = card_type.img_names.len();
                type_rc.borrow_mut().replace(card_type);
            }
        });
    }
    pub fn all_images(&self) -> Iter<String> {
        self.img_names.iter()
    }
    #[allow(non_snake_case)]
    pub fn T(&self) -> usize {
        self.T
    }
    pub fn size(&self) -> egui::Vec2 {
        if self.size.borrow().is_none() {
            if self.img_names.is_empty() {
                RefCell::replace(&self.size, Some(egui::Vec2::new(0.0, 0.0)));
            } else {
                let path = format!(
                    "http://127.0.0.1:8080/media/{folder}/{card}",
                    folder = self.path,
                    card = self.img_names[0]
                );
                RefCell::replace(&self.size, Uri(Cow::Owned(path)).texture_size());
            }
        }
        *self.size.borrow().as_ref().unwrap()
    }
}

pub trait FieldWidget {
    fn draw(&self) -> impl egui::Widget;
}

pub enum SimpleFieldKind {
    Stack,
    Horizontal,
}

pub struct SimpleField<C: CardWidget> {
    kind: SimpleFieldKind,
    cards: Vec<C>,
    pos: Option<egui::Pos2>,
    inner_margin: i8,
    max_cards: usize,
    selectable: bool,
    sense: egui::Sense,
    size: egui::Vec2,
}
impl<C: CardWidget> SimpleField<C> {
    // Builder
    pub fn new() -> Self {
        Self {
            kind: SimpleFieldKind::Horizontal,
            cards: vec![],
            pos: None,
            inner_margin: 5,
            max_cards: 5,
            selectable: false,
            sense: egui::Sense::empty(),
            size: egui::Vec2::new(100.0, 144.0),
        }
    }
    pub fn from_collection(cards: impl IntoIterator<Item = C>) -> Self {
        SimpleField {
            cards: cards.into_iter().collect(),
            ..SimpleField::new()
        }
    }
    pub fn pos(self, pos: egui::Pos2) -> Self {
        SimpleField {
            pos: Some(pos),
            ..self
        }
    }
    pub fn max_cards(self, max_cards: usize) -> Self {
        todo!()
    }
    pub fn kind(self, kind: SimpleFieldKind) -> Self {
        SimpleField { kind, ..self }
    }
    pub fn inner_margin(self, margin: i8) -> Self {
        SimpleField {
            inner_margin: margin,
            ..self
        }
    }
    pub fn selectable(self, selectable: bool) -> Self {
        SimpleField { selectable, ..self }
    }
    pub fn sense(self, sense: egui::Sense) -> Self {
        SimpleField { sense, ..self }
    }
}
impl<C: CardWidget> SimpleField<C> {
    fn card_pos(&self, idx: usize) -> egui::Vec2 {
        match self.kind {
            SimpleFieldKind::Stack => {
                let x = if idx <= self.max_cards {
                    idx as f32
                } else {
                    self.max_cards as f32
                };
                egui::Vec2::new(x, -x + self.inner_margin as f32)
            }
            SimpleFieldKind::Horizontal => {
                let cards = self.cards.len();
                let x = if cards <= self.max_cards {
                    (100.0 + self.inner_margin as f32) * (idx as f32)
                } else {
                    (self.size.x - 100.0) * (idx as f32) / (cards - 1) as f32
                };
                egui::Vec2::new(x, 0.0)
            }
        }
    }
    pub fn push(&mut self, card: C) {
        self.cards.push(card);
    }
    pub fn remove(&mut self, idx: usize) -> C {
        self.cards.remove(idx)
    }
    pub fn pop(&mut self) -> Option<C> {
        self.cards.pop()
    }
}
impl<C: CardWidget> FieldWidget for SimpleField<C> {
    fn draw(&self) -> impl egui::Widget {
        move |ui: &mut egui::Ui| -> egui::Response {
            frame::Frame::new()
                .inner_margin(egui::Margin::same(self.inner_margin))
                .outer_margin(egui::Margin::same(self.inner_margin))
                .stroke(egui::Stroke::new(2.0, egui::Color32::DEBUG_COLOR))
                .fill(egui::Color32::DARK_GREEN)
                .corner_radius(egui::CornerRadius::same(self.inner_margin.unsigned_abs()))
                .show(ui, |ui| {
                    let next_pos = ui.next_widget_position();
                    ui.allocate_new_ui(
                        egui::UiBuilder::new()
                            .max_rect(egui::Rect::from_min_size(next_pos, self.size))
                            .layer_id(egui::LayerId::background()),
                        |ui| {
                            ui.set_max_size(self.size);
                            ui.set_min_size(self.size);
                            let pointer = ui.input(|state| state.pointer.clone());
                            let mut selected = None;
                            if pointer.latest_pos().is_some()
                                && ui.max_rect().contains(pointer.latest_pos().unwrap())
                            {
                                let left = ui.max_rect().left();
                                let right = ui.max_rect().right();
                                let selector = self.cards.len() as f32
                                    * (pointer
                                        .latest_pos()
                                        .unwrap_or_else(|| egui::pos2(left, 0.0))
                                        .x
                                        - left)
                                    / (right - left);
                                selected = Some(selector as usize);
                            }
                            for (idx, card) in
                                self.cards.iter().enumerate().filter(|(idx, _)| {
                                    selected.is_none() || *idx != selected.unwrap()
                                })
                            {
                                let card_pos = next_pos.add(self.card_pos(idx));
                                ui.add(card.img());
                            }
                            if selected.is_some() {
                                if let Some(card) = self.cards.get(selected.unwrap()) {
                                    let card_pos = next_pos
                                        .add(self.card_pos(selected.unwrap()))
                                        .add(egui::vec2(0.0, -10.0));
                                    egui::Area::new(ui.next_auto_id())
                                        .order(egui::Order::Foreground)
                                        .sense(egui::Sense::all())
                                        .current_pos(card_pos)
                                        .show(ui.ctx(), |ui| {
                                            egui::Frame::new()
                                                .stroke(egui::Stroke::new(2.0, egui::Color32::RED))
                                                .corner_radius(egui::CornerRadius::same(2))
                                                .show(ui, |ui| {
                                                    ui.allocate_new_ui(
                                                        egui::UiBuilder::new(),
                                                        |ui| {
                                                            ui.add(card.img().maintain_aspect_ratio(true).max_size(self.size));
                                                        },
                                                    );
                                                });
                                        });
                                }
                            }
                        },
                    )
                    .response
                })
                .inner
        }
    }
}
