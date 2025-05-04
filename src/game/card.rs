#[cfg(target_arch = "wasm32")]
use crate::openDirectoryPicker;
use egui::ImageSource::Uri;
use egui::{frame, vec2, Rect};
use std::borrow::Cow;
use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use std::slice::Iter;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::js_sys::Array;
#[cfg(target_arch = "wasm32")]
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
    pub(crate) path: String,
    pub(crate) img_names: Vec<String>,
    pub(crate) T: usize,
    size: RefCell<Option<egui::Vec2>>,
}

impl DirectoryCardType {
    /// It's assumed the image URL is inside servers /media directory and the
    /// type order corresponds to the lexicographical.
    ///
    /// For real file upload you need to extend the simple python http server to accept uploads.
    /// Does pythons simple https server already accept POST requests?
    #[cfg(target_arch = "wasm32")]
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
#[allow(dead_code)]
pub struct SimpleField<C: CardWidget> {
    kind: SimpleFieldKind,
    cards: Vec<C>,
    pos: Option<egui::Pos2>,
    margin: i8,
    max_cards: usize,
    selectable: bool,
    sense: egui::Sense,
    pub(crate) size: egui::Vec2,
}
impl<C: CardWidget> SimpleField<C> {
    // Builder
    pub fn new() -> Self {
        Self {
            kind: SimpleFieldKind::Horizontal,
            cards: vec![],
            pos: None,
            margin: 4,
            max_cards: 5,
            selectable: false,
            sense: egui::Sense::empty(),
            size: egui::Vec2::new(100.0, 135.0),
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
        SimpleField { max_cards, ..self }
    }
    pub fn kind(self, kind: SimpleFieldKind) -> Self {
        SimpleField { kind, ..self }
    }
    pub fn margin(self, margin: i8) -> Self {
        SimpleField { margin, ..self }
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
                egui::Vec2::new(x, -x)
            }
            SimpleFieldKind::Horizontal => {
                let cards = self.cards.len();
                let x = if cards <= self.max_cards {
                    (self.size.x + self.margin as f32) * (idx as f32)
                } else {
                    (self.size.x + self.margin as f32)
                        * (idx as f32)
                        * ((self.max_cards - 1) as f32)
                        / ((cards - 1) as f32)
                };
                egui::Vec2::new(x, 0.0)
            }
        }
    }
    fn draw_stack(&self, ui: &mut egui::Ui) -> egui::Response {
        ui.set_min_size(self.size);
        let origin = ui
            .next_widget_position()
            .add(vec2(0.0, self.size.y / 2.0 + self.max_cards as f32));
        let content_size = self
            .size
            .add(vec2(self.max_cards as f32, self.max_cards as f32));
        ui.set_min_size(content_size);
        for (idx, card) in self.cards.iter().enumerate() {
            let img = card.img();
            if let Some(size) = img.load_and_calc_size(ui, self.size) {
                img.paint_at(
                    ui,
                    Rect::from_min_size(
                        origin.add(self.card_pos(idx).add(vec2(0.0, -size.y))),
                        size,
                    ),
                );
            }
        }
        ui.response()
    }
    fn draw_horizontal(&self, ui: &mut egui::Ui) -> egui::Response {
        let content_size = self.size.add(vec2(
            (self.max_cards as f32 - 1.0) * (self.size.x + self.margin as f32),
            0.0,
        ));
        ui.set_min_size(content_size);
        let origin = ui
            .next_widget_position()
            .add(vec2(0.0, self.size.y / 2.0));
        let pointer_pos = ui.input(|state| state.pointer.latest_pos());
        let rect = ui.min_rect();
        let selection: Option<usize> = if pointer_pos.is_some()
            && rect.contains(pointer_pos.unwrap())
        {
            let max = if self.cards.len() > self.max_cards {
                rect.right() - rect.left()
            } else {
                self.cards.len() as f32 * (self.size.x + self.margin as f32) - self.margin as f32
            };
            Some((self.cards.len() as f32 * (pointer_pos.unwrap().x - rect.left()) / max) as usize)
        } else {
            None
        };
        type Partition<'a, C> = (Vec<(usize, &'a C)>, Vec<(usize, &'a C)>);
        let (normal, selected): Partition<C> =
            self.cards.iter().enumerate().partition(|(i, _)| {
                !(self.selectable && (selection.is_some() && selection.unwrap() == *i))
            });
        for (idx, card) in normal {
            let img = card.img();
            if let Some(size) = img.load_and_calc_size(ui, self.size) {
                img.paint_at(
                    ui,
                    Rect::from_min_size(
                        origin.add(self.card_pos(idx).add(vec2(0.0, -size.y))),
                        size,
                    ),
                );
            }
        }
        for (idx, card) in selected {
            let img = card.img();
            if let Some(size) = img.load_and_calc_size(ui, self.size) {
                egui::Area::new(ui.next_auto_id())
                    .fixed_pos(
                        origin
                            .add(self.card_pos(idx))
                            .add(vec2(0.0, -size.y))
                            .add(vec2(0.0, -self.margin as f32)),
                    )
                    .show(ui.ctx(), |ui| {
                        egui::Frame::new()
                            .stroke(egui::Stroke::new(2.0, egui::Color32::RED))
                            .corner_radius(egui::CornerRadius::same(2))
                            .show(ui, |ui| {
                                ui.set_min_size(size);
                                img.paint_at(
                                    ui,
                                    Rect::from_min_size(
                                        origin
                                            .add(self.card_pos(idx))
                                            .add(vec2(0.0, -size.y))
                                            .add(vec2(
                                                self.margin as f32 / 2.0,
                                                -self.margin as f32 / 2.0,
                                            )),
                                        size,
                                    ),
                                );
                            });
                    });
            }
        }
        ui.response()
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
                .inner_margin(egui::Margin::same(self.margin))
                .stroke(egui::Stroke::new(2.0, egui::Color32::DEBUG_COLOR))
                .fill(egui::Color32::DARK_GREEN)
                .corner_radius(egui::CornerRadius::same(self.margin.unsigned_abs()))
                .show(ui, |ui| {
                    ui.allocate_ui(self.size, |ui| match self.kind {
                        SimpleFieldKind::Stack => self.draw_stack(ui),
                        SimpleFieldKind::Horizontal => self.draw_horizontal(ui),
                    })
                    .inner
                })
                .response
        }
    }
}

impl<C: CardWidget> Default for SimpleField<C> {
    fn default() -> Self {
        Self::new()
    }
}
