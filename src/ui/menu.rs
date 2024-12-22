pub mod components;
pub mod actions;
pub mod builder;

pub mod style {
    use bevy_egui::egui::Color32;

    pub const BUTTON_WIDTH: f32 = 200.0;
    pub const BUTTON_HEIGHT: f32 = 50.0;
    pub const SPACING: f32 = 10.0;
    pub const HEADING_SIZE: f32 = 48.0;
    pub const TEXT_SIZE: f32 = 24.0;

    pub const PRIMARY_COLOR: Color32 = Color32::WHITE;
    pub const BACKGROUND_COLOR: Color32 = Color32::from_rgb(70, 70, 70);
    pub const HOVER_COLOR: Color32 = Color32::from_rgb(120, 120, 120);
    pub const ACTIVE_COLOR: Color32 = Color32::from_rgb(50, 50, 50);
}