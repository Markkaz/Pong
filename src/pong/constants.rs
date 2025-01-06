 pub const WALL_THICKNESS: f32 = 10.0;
pub const TOP_BUFFER: f32 = 100.0;

 pub mod game {
     pub const MAX_SCORE: u32 = 5;
 }

pub mod paddle {
    pub const WIDTH: f32 = 10.0;
    pub const HEIGHT: f32 = 100.0;
    pub const BUFFER: f32 = 40.0;
    pub const SPEED: f32 = 6.;
}

pub mod ball {
    pub const RADIUS: f32 = 10.0;
    pub const INITIAL_VELOCITY: (f32, f32) = (200.0, 100.0);
    pub const SPEED_INCREASE: f32 = 1.1;
    pub const MAX_BALL_SPEED: f32 = 1000.;
}