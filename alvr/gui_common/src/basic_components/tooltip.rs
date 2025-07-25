use egui::{self, Ui, popup};

pub fn tooltip(ui: &mut Ui, id: &str, text: &str) {
    popup::show_tooltip_text(ui.ctx(), ui.layer_id(), egui::Id::new(id), text);
}
