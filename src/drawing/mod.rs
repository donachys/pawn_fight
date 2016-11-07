pub mod color {
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const DARKGREY: [f32; 4] = [0.35, 0.35, 0.35, 1.0];
    pub const LIGHTGREY: [f32; 4] = [0.85, 0.85, 0.85, 1.0];
    pub const BLUE: [f32; 4] = [0.05, 0.15, 0.9, 1.0];
    pub const BRIGHTBLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const VIOLET: [f32; 4] = [0.6, 0.0, 1.0, 1.0];
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const GREEN: [f32; 4] = [0.047, 0.55, 0.15, 1.0];
    pub const BRIGHTGREEN: [f32; 4] = [0.047, 0.95, 0.15, 1.0];
    pub const YELLOW: [f32; 4] = [0.9, 0.9, 0.15, 1.0];
}

pub mod screen {
    pub const WIDTH: i64 = 768;
    pub const HEIGHT: i64 = 768;
    pub const SIZE: i32 = 10;
}

pub mod token {
    pub const ARC_RESOLUTION: u32 = 128;
}
