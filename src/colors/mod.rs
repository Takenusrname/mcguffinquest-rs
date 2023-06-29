mod db16;
// Defaults
#[allow(unused)]
pub const DEFAULT_FG: (f32, f32, f32) = db16::DB16_LIGHT8_F32;
pub const DEFAULT_BG: (f32, f32, f32) = db16::DB16_DARK1_F32;

// Player
pub const PLAYER_FG: (f32, f32, f32) = db16::DB16_LIGHT7_F32;

// Mobs, NPC's, etc.
pub const ENEMY_FG: (f32, f32, f32) = db16::DB16_DARK7_F32;

// Screen burn
 // dark5, dark7, light1, light2, light4, light6 preferred
pub const SCREENBURN_COLOR: (u8, u8, u8) = db16::DB16_LIGHT1;