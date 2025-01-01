use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::ui::menu::components::MenuComponent;
use crate::ui::menu::style;

pub struct MenuBuilder {
    heading: String,
    top_spacing: f32,
    components: Vec<Box<dyn MenuComponent>>,
}

impl MenuBuilder {
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            heading: heading.into(),
            components: Vec::new(),
            top_spacing: 200.,
        }
    }

    pub fn with_top_spacing(mut self, top_spacing: f32) -> Self {
        self.top_spacing = top_spacing;
        self
    }

    pub fn add_component(mut self, component: impl MenuComponent + 'static) -> Self {
        self.components.push(Box::new(component));
        self
    }

    pub fn build(mut self, mut contexts: EguiContexts, commands: &mut Commands) {
        let ctx = contexts.ctx_mut();

        self.setup_style(ctx);
        self.render_menu(ctx, commands);
    }

    fn render_menu(&mut self, ctx: &egui::Context, commands: &mut Commands) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(self.top_spacing);

                ui.heading(egui::RichText::new(&self.heading)
                    .size(style::HEADING_SIZE)
                    .color(style::PRIMARY_COLOR)
                    .strong());
                ui.add_space(40.);

                for component in &mut self.components {
                    component.build(ui, commands);
                    ui.add_space(style::SPACING);
                }
            });
        });
    }

    fn setup_style(&self, ctx: &mut egui::Context) {
        ctx.style_mut(|style| {
            style.visuals.widgets.inactive = self.create_widget_style(style::BACKGROUND_COLOR);
            style.visuals.widgets.active = self.create_widget_style(style::ACTIVE_COLOR);
            style.visuals.widgets.hovered = self.create_widget_style(style::HOVER_COLOR);
        });
    }

    fn create_widget_style(&self, color: egui::Color32) -> egui::style::WidgetVisuals {
        egui::style::WidgetVisuals {
            bg_fill: color.clone(),
            weak_bg_fill: color.clone(),
            fg_stroke: egui::Stroke::NONE,
            bg_stroke: egui::Stroke::NONE,
            rounding: egui::Rounding::default(),
            expansion: 0.
        }
    }
}