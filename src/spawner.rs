use rltk::{ RGB, RandomNumberGenerator };
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::collections::HashMap;

use super::{ AreaOfEffect, BlocksTile, colors::*, CombatStats, Confusion, Consumable, DefenseBonus, EquipmentSlot, Equippable, EntryTrigger,
             glyph_index::*, Hidden, HungerClock, HungerState, InflictsDamage, Item, MagicMapper, map::MAPWIDTH, MeleePowerBonus, Monster, Name, Player,
             Position, ProvidesFood, ProvidesHealing, random_tables::RandomTable, Ranged, rect::Rect, Renderable, SerializeMe, SingleActivation, Viewshed, Map, TileType };

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
        .with(HungerClock{ state: HungerState::WellFed, duration: 20})
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

const MAX_MONSTERS: i32 = 4;

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new() 
        .add("Goblin", 10)
        .add("Orc", 1 + map_depth)
        .add("Health Potion", 7)
        .add("Fireball Scroll", 2 + map_depth)
        .add("Confusion Scroll", 2 + map_depth)
        .add("Magic Missile Scroll", 4)
        .add("Dagger", 3)
        .add("shield", 3)
        .add("Longsword", map_depth - 1)
        .add("Tower Shield", map_depth - 1) 
        .add("Rations", 10)
        .add("Magic Mapping Scroll", 2)
        .add("Bear Trap", 2)
}

pub fn spawn_room(ecs: &mut World, room: &Rect, map_depth: i32) {
    let mut possible_targets: Vec<usize> = Vec::new();
    {
        let map = ecs.fetch::<Map>();
        for y in room.y1 + 1 .. room.y2 {
            for x in room.x1 + 1 .. room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == TileType::Floor {
                    possible_targets.push(idx);
                }
            }
        }
    }
    spawn_region(ecs, &possible_targets, map_depth);
}

pub fn spawn_region(ecs: &mut World, area: &[usize], map_depth: i32) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);

    // Scope to keep borrow checker happy
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_spawns = i32::min(areas.len() as i32, rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3);
        if num_spawns == 0 { return; }

        for _i in 0..num_spawns {
            let array_index = if areas.len() == 1 { 0usize } else { (rng.roll_dice(1, areas.len() as i32) - 1) as usize };
            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll(&mut rng));
            areas.remove(array_index);
        }
    }

    // Actually spawn the monster
    for spawn in spawn_points.iter() {
        spawn_entity(ecs, &spawn);        
    }
}

fn spawn_entity(ecs: &mut World, spawn: &(&usize, &String)) {
    let x = (*spawn.0 % MAPWIDTH) as i32;
    let y = (*spawn.0 / MAPWIDTH) as i32;

    match spawn.1.as_ref() {
        "Goblin" => goblin(ecs, x, y),
        "Orc" => orc(ecs, x, y),
        "Health Potion" => health_potion(ecs, x, y),
        "Fireball Scroll" => fireball_scroll(ecs, x, y),
        "Confusion Scroll" => confusion_scroll(ecs, x, y),
        "Magic Missile Scroll" => magic_missile_scrolls(ecs, x, y),
        "Dagger" => dagger(ecs, x, y),
        "Shield" => shield(ecs, x, y),
        "Longsword" => longsword(ecs, x, y),
        "Tower Shield" => tower_shield(ecs, x, y),
        "Rations" => rations(ecs, x, y),
        "Magic Mapping Scroll" => magic_mapping_scroll(ecs, x, y),
        "Bear Trap" => bear_trap(ecs, x, y),
        _ => {}
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
        .marked::<SimpleMarker<SerializeMe>>()
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
        .marked::<SimpleMarker<SerializeMe>>()
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
        .marked::<SimpleMarker<SerializeMe>>()
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
        .marked::<SimpleMarker<SerializeMe>>()
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
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn dagger(ecs: &mut World, x: i32, y: i32) {
    let glyph: u16 = rltk::to_cp437(DAGGER_GLYPH);
    let fg: RGB = return_rgb(DAGGER_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
    let name: &str = "Dagger";
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable {
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name{ name: name.to_string() })
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn shield(ecs: &mut World, x: i32, y: i32) {
    let glyph: u16 = rltk::to_cp437(SHIELD_GLYPH);
    let fg: RGB = return_rgb(SHIELD_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
    let name: &str = "Shield";
    
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name { name: name.to_string() })
        .with(Item {})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenseBonus{ defense: 1})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn longsword(ecs: &mut World, x: i32, y: i32) {
    
    let glyph: u16 = rltk::to_cp437(SWORD_GLYPH);
    let fg: RGB = return_rgb(SWORD_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
    let name: &str = "Longsword";

    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name { name: name.to_string()})
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 4})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn tower_shield(ecs: &mut World, x: i32, y: i32) {

    let glyph: u16 = rltk::to_cp437(TOWER_S_GLYPH);
    let fg: RGB = return_rgb(TOWER_S_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
    let name: &str = "Tower Shield";

    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name { name: name.to_string()})
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenseBonus{ defense: 3})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn rations(ecs: &mut World, x: i32, y: i32) {

    let glyph: u16 = rltk::to_cp437(RATIONS_GLYPH);
    let fg: RGB = return_rgb(RATION_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
    let name: &str = "Rations";

    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name{ name: name.to_string() })
        .with(Item{})
        .with(ProvidesFood{})
        .with(Consumable{})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn magic_mapping_scroll(ecs: &mut World, x: i32, y: i32) {
    
    let glyph: u16 = rltk::to_cp437(SCROLL_GLYPH);
    let fg: RGB = return_rgb(MAGICMAP_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
    let name: &str = "Scroll of Magic Mapping";

    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name{ name: name.to_string() })
        .with(Item{})
        .with(MagicMapper{})
        .with(Consumable{})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn bear_trap(ecs: &mut World, x: i32, y: i32) {

    let glyph: u16 = rltk::to_cp437(BEARTRAP_GLYPH);
    let fg: RGB = return_rgb(BEARTRAP_FG);
    let bg: RGB = return_rgb(DEFAULT_BG);
    let name: &str = "Bear Trap";

    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg,
            bg,
            render_order: 2
        })
        .with(Name{ name: name.to_string() })
        .with(Hidden{})
        .with(EntryTrigger{})
        .with(InflictsDamage{ damage: 6 })
        .with(SingleActivation {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}