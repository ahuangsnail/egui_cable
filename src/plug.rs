use egui::{pos2, Id, Order, Pos2, Rect, Sense, Vec2, Widget};

use crate::{
    cable::CableId,
    event::Event,
    state::State,
    utils::{widget_visuals, SIZE},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlugType {
    In,
    Out,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PlugId {
    cable_id: CableId,
    plug_type: PlugType,
}

impl PlugId {
    pub fn new(cable_id: CableId, plug_type: PlugType) -> Self {
        PlugId {
            cable_id,
            plug_type,
        }
    }
}

#[derive(Debug, Default)]
pub struct Plug {
    pub plug_to: Option<Id>,
    pos: Option<Pos2>,
    // inserted by Cable widget
    id: Option<PlugId>,
    // inserted by Cable widget
    default_pos: Option<Pos2>,
}

#[derive(Debug, Clone)]
pub(crate) struct DraggedPlug {
    pub pos: Pos2,
    pub size: Vec2,
}

impl Plug {
    pub fn new() -> Self {
        Plug::default()
    }

    pub fn plug_to(port: Id) -> Self {
        Plug {
            plug_to: Some(port),
            ..Default::default()
        }
    }

    pub(crate) fn default_pos(mut self, pos: Pos2) -> Self {
        self.default_pos = Some(pos);
        self
    }

    pub fn pos(mut self, pos: Pos2) -> Self {
        self.pos = Some(pos);
        self
    }

    pub(crate) fn id(mut self, id: PlugId) -> Self {
        self.id = Some(id);
        self
    }
}

impl Widget for Plug {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = self.id.unwrap();
        let mut state = State::get_cloned(ui.data());
        let mut pos = if let Some(port_id) = &self.plug_to {
            state
                .port_pos(port_id)
                // If port is not displayed, use saved plug pos
                .or_else(|| state.plug_pos(&id))
                // FIXME: pos2(0.0, 0.0) is not good.
                .unwrap_or(pos2(0.0, 0.0))
        } else {
            state
                .plug_pos(&id)
                .unwrap_or_else(|| self.default_pos.unwrap())
        };
        egui::Area::new(id.clone())
            // must be top-left of the widget
            .current_pos(pos)
            // should be displayed on foreground
            .order(Order::Foreground)
            .show(ui.ctx(), |ui| {
                let response = if self.plug_to.is_some() {
                    let (_rect, response) =
                        ui.allocate_exact_size(SIZE, Sense::focusable_noninteractive());
                    response
                } else {
                    let response =
                        ui.allocate_rect(Rect::from_two_pos(pos, pos + SIZE), Sense::drag());
                    let size = response.rect.size();

                    pos += response.drag_delta();

                    let center_pos = pos + size / 2.0;

                    // Update plug pos used for determining a port is hovered by plug
                    if response.dragged() {
                        state.update_dragged_plug(DraggedPlug {
                            pos: center_pos,
                            size: response.rect.size(),
                        });
                    }

                    if response.drag_released() {
                        // Connect event
                        if let Some(port_id) = state.hovered_port_id() {
                            state.ephemeral.event.insert(
                                id.cable_id,
                                Event::Connected {
                                    plug_type: id.plug_type,
                                    port_id,
                                },
                            );
                        }
                    }

                    // paint circles
                    let visuals = widget_visuals(ui, &response);
                    ui.painter().add(epaint::CircleShape {
                        center: center_pos,
                        radius: size.x / 2.0,
                        fill: visuals.bg_fill,
                        stroke: visuals.fg_stroke,
                    });
                    ui.painter().add(epaint::CircleShape {
                        center: center_pos,
                        radius: size.x / 4.0,
                        fill: visuals.bg_fill,
                        stroke: visuals.fg_stroke,
                    });

                    response
                };

                state.update_plug_pos(id.clone(), pos);
                state.store_to(ui.data());

                response
            })
            .inner
    }
}
