mod db16;
use db16::*;
// Defaults
#[allow(unused)]
pub const DEFAULT_FG: (f32, f32, f32) = DB16_LIGHT8_F32;
pub const DEFAULT_BG: (f32, f32, f32) = DB16_DARK1_F32;

// Screen burn
// dark5, dark7, light1, light2, light4, light6 preferred
pub const SCREENBURN_COLOR: (u8, u8, u8) = DB16_LIGHT6;

// Player
pub const PLAYER_FG: (f32, f32, f32) = DB16_LIGHT8_F32;

// Mobs, NPC's, etc.
pub const ENEMY_FG: (f32, f32, f32) = DB16_DARK7_F32;

// Dungeon
pub const WALL_COLOR: (f32, f32, f32) = DB16_LIGHT5_F32;
pub const FLOOR_COLOR: (f32, f32, f32) = DB16_DARK5_F32;
pub const OUT_OF_VIEW: (f32, f32, f32) = DB16_DARK2_F32;
pub const AETHER_FG: (f32,f32,f32) = DB16_DARK4_F32;

// User Interface
pub const HEALTH_BAR_FG: (f32, f32, f32) = DB16_DARK7_F32;
pub const MOUSE_BG: (f32, f32, f32) = DB16_LIGHT6_F32;
pub const TOOLTIP_BG: (f32, f32, f32) = DB16_DARK8_F32;
