use rltk::{ RGB, RandomNumberGenerator };
use specs::prelude::*;

use super::{ AreaOfEffect, BlocksTile, colors::*, CombatStats, Confusion, Consumable, glyph_index::*,
             InflictsDamage, Item, map::MAPWIDTH, Monster, Name, Player, Position, ProvidesHealing,
             Ranged, rect::Rect, Renderable, Viewshed };

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

/// Spawn the player and returns his/her entity object.
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    let fore_col: RGB = RGB::from_f32(PLAYER_FG.0,PLAYER_FG.1,PLAYER_FG.2);
    let back_col: RGB = RGB::from_f32(DEFAULT_BG.0,DEFAULT_BG.1,DEFAULT_BG.2);
    let player_glyph: u16 = rltk::to_cp437(PLAYER_GLYPH);
    ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y})
        .with(Renderable {
            glyph: player_glyph,
            fg: fore_col,
            bg: back_col,
            render_order: 0
        })
        .with(Player{})
        .with(Viewshed{
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true
        })
        .with(Name{ name: "Player".to_string() })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5
        })
        .build()
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => { orc(ecs, x, y) }
        _ => { goblin(ecs, x, y)}
    }
}

fn orc(ecs: &mut World, x: i32, y: i32) {
    let glyph: u16 = rltk::to_cp437(ORC_GLYPH);
    let fg: RGB = RGB::from_f32(ORC_FG.0, ORC_FG.1, ORC_FG.2);
    let name: &str = "Orc";

    monster(ecs, x, y, glyph, fg, name);
}

fn goblin(ecs: &mut World, x: i32, y: i32) {
    let glyph: u16 = rltk::to_cp437(GOBLIN_GLYPH);
    let fg: RGB = RGB::from_f32(GOBLIN_FG.0, GOBLIN_FG.1, GOBLIN_FG.2);
    let name: &str = "Goblin";

    monster(ecs, x, y, glyph, fg, name);
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: rltk::FontCharType, fg: RGB, name: S) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg,
            bg: RGB::from_f32(DEFAULT_BG.0, DEFAULT_BG.1, DEFAULT_BG.2),
            render_order: 1
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true
        })
        .with(Monster{})
        .with(Name{ name: name.to_string() })
        .with(BlocksTile{})
        .with(CombatStats{
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4
        })
        .build();
}

fn health_potion(ecs: &mut World, x: i32, y: i32) {
    let glyph: u16 = rltk::to_cp437(POTION_GLYPH);
    let fg: RGB = RGB::from_f32(HEALTH_POT_FG.0, HEALTH_POT_FG.1, HEALTH_POT_FG.2);
    let bg: RGB = RGB::from_f32(DEFAULT_BG.0, DEFAULT_BG.1, DEFAULT_BG.2);
    let name: &str = "Health Potion";
    
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable {
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name { name: name.to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesHealing { heal_amount: 8})
        .build();
}

fn magic_missile_scrolls(ecs: &mut World, x: i32, y: i32) {
    let glyph: u16 = rltk::to_cp437(SCROLL_GLYPH);
    let fg: RGB = RGB::from_f32(MMS_FG.0, MMS_FG.1, MMS_FG.2);
    let bg: RGB = RGB::from_f32(DEFAULT_BG.0, DEFAULT_BG.1, DEFAULT_BG.2);
    
    ecs.create_entity()
        .with(Position{ x, y})
        .with(Renderable{
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name{ name: "Magic Missle Scroll".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .build();
}

fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {

    let glyph: u16 = rltk::to_cp437(SCROLL_GLYPH);
    let fg: RGB = RGB::from_f32(FIREBALL_FG.0, FIREBALL_FG.1, FIREBALL_FG.2);
    let bg: RGB = RGB::from_f32(DEFAULT_BG.0, DEFAULT_BG.1, DEFAULT_BG.2);

    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name{ name: "FireBall Scroll".to_string()})
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{range: 6})
        .with(InflictsDamage{damage: 20})
        .with(AreaOfEffect{radius: 3})
        .build();
}

fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {

    let glyph: u16 = rltk::to_cp437(SCROLL_GLYPH);
    let fg: RGB = RGB::from_f32(CONFUSION_FG.0, CONFUSION_FG.1, CONFUSION_FG.2);
    let bg: RGB = RGB::from_f32(DEFAULT_BG.0, DEFAULT_BG.1, DEFAULT_BG.2);

    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name{ name: "Confusion Scroll".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{range: 6})
        .with(Confusion{ turns: 4 })
        .build();
}

fn random_item(ecs: &mut World, x: i32, y: i32){
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 4);
    }
    match roll {
        1 => { health_potion(ecs, x, y) }
        2 => { fireball_scroll(ecs, x, y) }
        3 => { confusion_scroll(ecs, x, y) }
        _ => { magic_missile_scrolls(ecs, x, y) }
    }
}

pub fn spawn_room(ecs: &mut World, room: &Rect) {
    let mut monster_spawn_points: Vec<usize> = Vec::new();
    let mut item_spawn_points: Vec<usize> = Vec::new();

    // Scope to keep borrow checker happy
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;

        for _i in 0..num_monsters {
            let mut added = false;

            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;

                let idx = (y * MAPWIDTH) + x;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        for _i in 0..num_items {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !item_spawn_points.contains(&idx) {
                    item_spawn_points.push(idx);
                    added = true;
                }
            }
        }
    }

    // Actually spawn the monster
    for idx in monster_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(ecs, x as i32, y as i32);
    }

    // Actually spawn the potions
    for idx in item_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_item(ecs, x as i32, y as i32);
    }
}