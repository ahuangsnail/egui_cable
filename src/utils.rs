use egui::{style::WidgetVisuals, Response, Ui};

pub fn visual(ui: &mut Ui, response: &Response) -> WidgetVisuals {
    if response.hovered() {
        return ui.visuals().widgets.hovered;
    };
    if response.dragged() {
        return ui.visuals().widgets.active;
    }
    ui.visuals().widgets.inactive
}