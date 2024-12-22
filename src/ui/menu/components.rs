use bevy::prelude::*;
use bevy_egui::egui;
use crate::ui::menu::actions::MenuAction;
use crate::ui::menu::style;

pub trait MenuComponent {
    fn build(&mut self, ui: &mut egui::Ui, commands: &mut Commands);
}

pub struct MenuLabel {
    label: String,
}

impl MenuLabel {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into() }
    }
}

impl MenuComponent for MenuLabel {
    fn build(&mut self, ui: &mut egui::Ui, _commands: &mut Commands) {
        ui.add_sized(
            egui::Vec2::new(style::BUTTON_WIDTH, style::BUTTON_HEIGHT),
            egui::Label::new(
                egui::RichText::new(&self.label)
                    .color(style::PRIMARY_COLOR)
                    .size(style::TEXT_SIZE),
            ),
        );
    }
}

pub struct MenuSelectableLabel {
    label: String,
    selected: bool,
    action: Box<dyn MenuAction>,
}

impl MenuSelectableLabel {
    pub fn new(
        label: impl Into<String>,
        selected: bool,
        action: impl MenuAction + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            selected,
            action: Box::new(action),
        }
    }
}

impl MenuComponent for MenuSelectableLabel {
    fn build(&mut self, ui: &mut egui::Ui, commands: &mut Commands) {
        if ui.add_sized(
            egui::Vec2::new(style::BUTTON_WIDTH, style::BUTTON_HEIGHT),
            egui::SelectableLabel::new(
                self.selected,
                egui::RichText::new(&self.label).size(style::TEXT_SIZE).color(style::PRIMARY_COLOR)
            )
        ).clicked() {
            self.action.execute(commands);
        }
    }
}

pub struct MenuButton {
    label: String,
    action: Box<dyn MenuAction>,
}

impl MenuButton {
    pub fn new(label: impl Into<String>, action: impl MenuAction + 'static) -> Self {
        Self {
            label: label.into(),
            action: Box::new(action),
        }
    }
}

impl MenuComponent for MenuButton {
    fn build(&mut self, ui: &mut egui::Ui, commands: &mut Commands) {
        if ui.add_sized(
            egui::Vec2::new(style::BUTTON_WIDTH, style::BUTTON_HEIGHT),
            egui::Button::new(
                egui::RichText::new(&self.label)
                    .size(style::TEXT_SIZE)
                    .color(style::PRIMARY_COLOR)
            )
        ).clicked() {
            self.action.execute(commands);
        }
    }
}

pub struct MenuLayoutHorizontal {
    components: Vec<Box<dyn MenuComponent>>,
}

impl MenuLayoutHorizontal {
    pub fn new() -> Self {
        Self { components: Vec::new() }
    }

    pub fn add_component(mut self, component: impl MenuComponent + 'static) -> Self {
        self.components.push(Box::new(component));
        self
    }
}

impl MenuComponent for MenuLayoutHorizontal {
    fn build(&mut self, ui: &mut egui::Ui, commands: &mut Commands) {
        let layout = egui::Layout::centered_and_justified(egui::Direction::LeftToRight);
        ui.allocate_ui_with_layout(
            [self.components.len() as f32 * (style::BUTTON_WIDTH + 5.), style::BUTTON_HEIGHT].into(),
            layout,
            |ui| {
                ui.horizontal(|ui| {
                    for component in &mut self.components {
                        component.build(ui, commands);
                    }
                });
            });
    }
}