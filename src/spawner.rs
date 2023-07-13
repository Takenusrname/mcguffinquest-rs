use rltk::{ RGB, RandomNumberGenerator };
use specs::prelude::*;

use super::{ AreaOfEffect, BlocksTile, colors::*, CombatStats, Confusion, Consumable, glyph_index::*,
             InflictsDamage, Item, map::MAPWIDTH, Monster, Name, Player, Position, ProvidesHealing,
             Ranged, rect::Rect, Renderable, Viewshed };

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

/// Spawn the player and returns his/her entity object.
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    let fg: RGB = return_rgb(PLAYER_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
    let glyph: u16 = rltk::to_cp437(PLAYER_GLYPH);
    ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y})
        .with(Renderable {
            glyph,
            fg,
            bg,
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
    let fg: RGB = return_rgb(ORC_FG);
    let name: &str = "Orc";

    monster(ecs, x, y, glyph, fg, name);
}

fn goblin(ecs: &mut World, x: i32, y: i32) {
    let glyph: u16 = rltk::to_cp437(GOBLIN_GLYPH);
    let fg: RGB = return_rgb(GOBLIN_FG);
    let name: &str = "Goblin";

    monster(ecs, x, y, glyph, fg, name);
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: rltk::FontCharType, fg: RGB, name: S) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg,
            bg: return_rgb(DEFAULT_BG),
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
    let fg: RGB = return_rgb(HEALTH_POT_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
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
    let fg: RGB = return_rgb(MMS_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
    
    ecs.create_entity()
        .with(Position{ x, y})
        .with(Renderable{
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name{ name: "Magic Missile Scroll".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .build();
}

fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {

    let glyph: u16 = rltk::to_cp437(SCROLL_GLYPH);
    let fg: RGB = return_rgb(FIREBALL_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);

    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name{ name: "Fireball Scroll".to_string()})
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{range: 6})
        .with(InflictsDamage{damage: 20})
        .with(AreaOfEffect{radius: 3})
        .build();
}

fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {

    let glyph: u16 = rltk::to_cp437(SCROLL_GLYPH);
    let fg: RGB = return_rgb(CONFUSION_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);

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