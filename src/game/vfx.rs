use crate::game::card::Drawable;

pub trait Field {
    fn ui(&self, ui: &mut egui::Ui) -> egui::Response;
    fn pos(&self) -> egui::Pos2;
    fn set_pos(&mut self, pos: egui::Pos2);
}

impl egui::Widget for &dyn Field {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        
        self.ui(ui)
    }
}

impl Drawable for &dyn Field {
    fn pos(&self) -> egui::Pos2 {
        (*self).pos()
    }
}
