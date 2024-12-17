use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Color32;

pub trait MenuAction {
    fn execute(&self, commands: &mut Commands);
}

pub struct ChangeStateMenuAction<State: FreelyMutableState> {
    next_state: State,
}

impl<State: FreelyMutableState> ChangeStateMenuAction<State> {
    pub fn new(next_state: State) -> Self {
        Self { next_state }
    }
}

impl<State: FreelyMutableState> MenuAction for ChangeStateMenuAction<State> {
    fn execute(&self, commands: &mut Commands) {
        commands.set_state(self.next_state.clone());
    }
}

pub struct UpdateResourceMenuAction<R: Resource + Copy> {
    resource: R,
}

impl<R: Resource + Copy> UpdateResourceMenuAction<R> {
    pub fn new(resource: R) -> Self {
        Self { resource }
    }
}

impl<R: Resource + Copy> MenuAction for UpdateResourceMenuAction<R> {
    fn execute(&self, commands: &mut Commands) {
        commands.insert_resource(self.resource);
    }
}

pub struct QuitMenuAction;

impl MenuAction for QuitMenuAction {
    fn execute(&self, commands: &mut Commands) {
        commands.send_event(AppExit::Success);
    }
}

pub struct MenuBuilder {
    heading: String,
    components: Vec<Box<dyn MenuComponent>>,
}

impl MenuBuilder {
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            heading: heading.into(),
            components: Vec::new(),
        }
    }

    pub fn add_component(mut self, component: impl MenuComponent + 'static) -> Self {
        self.components.push(Box::new(component));
        self
    }

    pub fn build(mut self, mut contexts: EguiContexts, commands: &mut Commands) {
        let ctx = contexts.ctx_mut();

        ctx.style_mut(|style| {
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(70, 70, 70);
            style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(70, 70, 70);
            style.visuals.widgets.inactive.fg_stroke = egui::Stroke::NONE;
            style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;


            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(50, 50, 50);
            style.visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(50, 50, 50);
            style.visuals.widgets.active.fg_stroke = egui::Stroke::NONE;
            style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(120, 120, 120);
            style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(120, 120, 120);
            style.visuals.widgets.hovered.fg_stroke = egui::Stroke::NONE;
            style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(200.);

                ui.heading(egui::RichText::new(&self.heading)
                    .size(48.)
                    .color(egui::Color32::WHITE)
                    .strong());
                ui.add_space(40.);

                for component in &mut self.components {
                    component.build(ui, commands);
                    ui.add_space(10.);
                }
            });
        });
    }
}

pub trait MenuComponent {
    fn build(&mut self, ui: &mut egui::Ui, commands: &mut Commands);
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
            [self.components.len() as f32 * 205., 50.].into(),
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
            egui::Vec2::new(200., 50.),
            egui::Label::new(egui::RichText::new(&self.label).color(Color32::WHITE).size(24.)),
        );
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
            egui::Vec2::new(200., 50.),
            egui::Button::new(
                egui::RichText::new(&self.label)
                    .size(24.)
                    .color(egui::Color32::WHITE)
            )
        ).clicked() {
            self.action.execute(commands);
        }
    }
}

pub struct MenuOptions {
    label: String,
    options: Vec<MenuChoice>,
}

impl MenuOptions {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), options: Vec::new() }
    }

    pub fn add_option(mut self, label: impl Into<String>, selected: bool, action: impl MenuAction + 'static) -> Self {
        self.options.push(
            MenuChoice {
                label: label.into(),
                selected,
                action: Box::new(action),
            }
        );
        self
    }
}

impl MenuComponent for MenuOptions {
    fn build(&mut self, ui: &mut egui::Ui, commands: &mut Commands) {
        let layout = egui::Layout::centered_and_justified(egui::Direction::LeftToRight);
        ui.allocate_ui_with_layout(
            [(self.options.len() as f32 + 1.) * 205., 50.].into(),
            layout,
            |ui| {
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [200., 50.],
                        egui::Label::new(egui::RichText::new(&self.label).size(24.).color(egui::Color32::WHITE))
                    );

                    for option in &mut self.options {
                        option.render(ui, commands);
                    }
                });
            });

        ui.add_space(20.);
    }
}

struct MenuChoice {
    label: String,
    selected: bool,
    action: Box<dyn MenuAction>,
}

impl MenuChoice {
    fn render(&mut self, ui: &mut egui::Ui, commands: &mut Commands) {
        if ui.add_sized(
            egui::Vec2::new(200., 50.),
            egui::SelectableLabel::new(
                self.selected,
                egui::RichText::new(&self.label).size(24.).color(egui::Color32::WHITE)
            )
        ).clicked() {
            self.action.execute(commands);
        }
    }
}