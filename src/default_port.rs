use egui::{Response, Sense, Widget};

use crate::{
    port_params::PortParams,
    utils::{widget_visuals, SIZE},
};

#[derive(Debug)]
pub struct DefaultPort;

impl Widget for DefaultPort {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let params = PortParams::get(ui.data());
        let hovered = params.hovered;

        // minimum sense because interaction of port is not needed for now
        let (rect, response) = ui.allocate_exact_size(SIZE, Sense::hover());

        // paint the port
        let visuals = if hovered {
            ui.visuals().widgets.hovered
        } else {
            widget_visuals(ui, &response)
        };
        ui.painter().add(epaint::CircleShape {
            center: rect.center(),
            radius: rect.height() / 2.0,
            fill: visuals.bg_fill,
            stroke: visuals.fg_stroke,
        });
        response
    }
}
