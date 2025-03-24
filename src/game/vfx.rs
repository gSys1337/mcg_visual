pub trait Field {
    fn ui(&self, ui: &mut egui::Ui) -> egui::Response;
    fn set_pos(&mut self, pos: egui::Pos2);
}

impl egui::Widget for &dyn Field {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.ui(ui)
    }
}
