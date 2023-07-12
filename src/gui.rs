use rltk::{ Point, RGB, Rltk, VirtualKeyCode};
use specs::prelude::*;

use rltk::Rect;

use super::colors::*;
use super::{ CombatStats, game_log::GameLog, InBackpack, Map, Name, Player, Position, State, Viewshed };

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    let fg: (f32, f32, f32) = DEFAULT_FG;
    let bg: (f32, f32, f32) = DEFAULT_BG;

    let m_bg: (f32, f32, f32) = MOUSE_BG;

    let info_title = format!(" Player Info. ");
    let msg_title = format!(" Message Log ");

    let bg_rect = Rect::with_size(0, 40, 79, 49);

    ctx.fill_region(bg_rect, rltk::to_cp437(' '), RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2));

    ctx.draw_hollow_box(0, 40, 23, 9, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2));
    ctx.draw_hollow_box(23, 40, 79-23, 9, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2));

    ctx.set(23, 40, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), rltk::to_cp437('┬'));
    ctx.set(23, 49, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), rltk::to_cp437('┴'));

    ctx.set(1, 40, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), rltk::to_cp437('┤'));
    ctx.print_color(2, 40, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), info_title);
    ctx.set(16, 40, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), rltk::to_cp437('├'));

    ctx.set(24, 40, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), rltk::to_cp437('┤'));
    ctx.print_color(25, 40, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), msg_title);
    ctx.set(38, 40, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), rltk::to_cp437('├'));
    
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();

    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!("HP: {} / {} ", stats.hp, stats.max_hp);        
        
        ctx.print_color(2, 42, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), &health);

        let bar_fg: (f32, f32, f32) = HEALTH_BAR_FG;
        let bar_bg: (f32, f32, f32) = DEFAULT_BG;

        ctx.draw_bar_horizontal(2, 43, 20, stats.hp, stats.max_hp, RGB::from_f32(bar_fg.0, bar_fg.1, bar_fg.2), RGB::from_f32(bar_bg.0, bar_bg.1, bar_bg.2))
    }

    let log = ecs.fetch::<GameLog>();

    let mut y = 41;
    for s in log.entries.iter().rev() {
        if y < 49 { ctx.print_color(25, y, RGB::from_f32(fg.0, fg.1, fg.2), RGB::from_f32(bg.0, bg.1, bg.2), s);}
        y += 1;
    }

    // Draw Mouse Cursor
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::from_f32(m_bg.0, m_bg.1, m_bg.2));
    draw_tooltips(ecs, ctx);
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height { return; }
    let mut tooltip: Vec<String> = Vec::new();
    for (name, position) in (&names, &positions).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 { width = s.len() as i32; }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x, y, RGB::from_f32(DEFAULT_FG.0, DEFAULT_FG.1, DEFAULT_FG.2), RGB::from_f32(TOOLTIP_BG.0, TOOLTIP_BG.1, TOOLTIP_BG.2), s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x - i, y, RGB::from_f32(DEFAULT_FG.0, DEFAULT_FG.1, DEFAULT_FG.2), RGB::from_f32(TOOLTIP_BG.0, TOOLTIP_BG.1, TOOLTIP_BG.2), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::from_f32(DEFAULT_FG.0, DEFAULT_FG.1, DEFAULT_FG.2), RGB::from_f32(TOOLTIP_BG.0, TOOLTIP_BG.1, TOOLTIP_BG.2), &"->".to_string());
        } else {
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y  = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x + 1, y, RGB::from_f32(DEFAULT_FG.0, DEFAULT_FG.1, DEFAULT_FG.2), RGB::from_f32(TOOLTIP_BG.0, TOOLTIP_BG.1, TOOLTIP_BG.2), s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x + 1 + i, y, RGB::from_f32(DEFAULT_FG.0, DEFAULT_FG.1, DEFAULT_FG.2), RGB::from_f32(TOOLTIP_BG.0, TOOLTIP_BG.1, TOOLTIP_BG.2), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::from_f32(DEFAULT_FG.0, DEFAULT_FG.1, DEFAULT_FG.2), RGB::from_f32(TOOLTIP_BG.0, TOOLTIP_BG.1, TOOLTIP_BG.2), &"<-".to_string());
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult { Cancel, NoResponse, Selected }

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names).join().filter(|item| item.0.owner == *player_entity );
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;

    let bg_rect = Rect::with_size(15, y - 2, 31, (count + 3) as i32);

    let fg: RGB = RGB::from_f32(MENU_FG.0, MENU_FG.1, MENU_FG.2);
    let bg: RGB = RGB::from_f32(INV_BG.0, INV_BG.1, INV_BG.2);
    let ctrl_fg: RGB = RGB::from_f32(CTRL_FG.0, CTRL_FG.1, CTRL_FG.2);

    let start_char = rltk::to_cp437('┤');
    let end_char = rltk::to_cp437('├');

    ctx.fill_region(bg_rect, rltk::to_cp437(' '), fg, bg);
    ctx.draw_hollow_box(15, y - 2, 31, (count + 3) as i32, fg, bg);
    ctx.print_color(16, y - 2, bg, fg, " Inventory ");
    ctx.print_color(30, y + count as i32 + 1, ctrl_fg, bg, " ESC ");
    ctx.print_color(35, y + count as i32 + 1, fg, bg, "to cancel ");
    ctx.set(29, y + count as i32 + 1, fg, bg, start_char);
    ctx.set(45, y + count as i32 + 1, fg, bg, end_char);

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity) {
        ctx.set(17, y, fg, bg, rltk::to_cp437('('));
        ctx.set(18, y, ctrl_fg, bg, 97 + j as rltk::FontCharType);
        ctx.set(19, y, fg, bg, rltk::to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1; 
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        return (ItemMenuResult::Selected, Some(equippable[selection as usize]));
                    }
                    (ItemMenuResult::NoResponse, None)
                }
            }
        }
    }
    
}

pub fn drop_item_menu(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names).join().filter(|item| item.0.owner == *player_entity );
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;

    let bg_rect = Rect::with_size(15, y - 2, 31, (count + 3) as i32);

    let fg: RGB = RGB::from_f32(MENU_FG.0, MENU_FG.1, MENU_FG.2);
    let bg: RGB = RGB::from_f32(DROP_BG.0, DROP_BG.1, DROP_BG.2);
    let ctrl_fg: RGB = RGB::from_f32(CTRL_FG.0, CTRL_FG.1, CTRL_FG.2);

    let start_char = rltk::to_cp437('┤');
    let end_char = rltk::to_cp437('├');

    ctx.fill_region(bg_rect, rltk::to_cp437(' '), fg, bg);
    ctx.draw_hollow_box(15, y - 2, 31, (count + 3) as i32, fg, bg);
    ctx.print_color(16, y - 2, bg, fg, " Drop Which Item? ");
    ctx.print_color(30, y + count as i32 + 1, ctrl_fg, bg, " ESC ");
    ctx.print_color(35, y + count as i32 + 1, fg, bg, "to cancel ");
    ctx.set(29, y + count as i32 + 1, fg, bg, start_char);
    ctx.set(45, y + count as i32 + 1, fg, bg, end_char);

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity) {
        ctx.set(17, y, fg, bg, rltk::to_cp437('('));
        ctx.set(18, y, ctrl_fg, bg, 97 + j as rltk::FontCharType);
        ctx.set(19, y, fg, bg, rltk::to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1; 
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        return (ItemMenuResult::Selected, Some(equippable[selection as usize]));
                    }
                    (ItemMenuResult::NoResponse, None)
                }
            }
        }
    }
}

pub fn ranged_target(gs: &mut State, ctx: &mut Rltk, range: i32) -> (ItemMenuResult, Option<Point>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let player_pos = gs.ecs.fetch::<Point>();
    let viewsheds = gs.ecs.read_storage::<Viewshed>();

    // Targeting message
    let fg: RGB = RGB::from_f32(CTRL_FG.0, CTRL_FG.1, CTRL_FG.2);
    let bg: RGB = RGB::from_f32(DEFAULT_BG.0, DEFAULT_BG.1, DEFAULT_BG.2);
    let msg = "Select Target";
    ctx.print_color(5, 0, fg, bg, msg);

    // Highlight available target cells
    let target_bg: RGB = RGB::from_f32(TARGET_BG.0, TARGET_BG.1, TARGET_BG.2);
    let mut available_cells = Vec::new();
    let visible = viewsheds.get(*player_entity);
    if let Some(visible) = visible {
        // We have a viewshed
        for idx in visible.visible_tiles.iter() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if distance <= range as f32 {
                ctx.set_bg(idx.x, idx.y, target_bg);
                available_cells.push(idx);
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    // Draw mouse cursor
    let valid_bg: RGB = RGB::from_f32(MOUSE_BG.0, MOUSE_BG.1, MOUSE_BG.2);
    let invalid_bg: RGB = RGB::from_f32(ERROR_BG.0, ERROR_BG.1, ERROR_BG.2);
    let mouse_pos = ctx.mouse_pos();
    let mut valid_target = false;
    for idx in available_cells.iter() { if idx.x == mouse_pos.0 && idx.y == mouse_pos.1 { valid_target = true; } }
    if valid_target {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, valid_bg);
        if ctx.left_click {
            return (ItemMenuResult::Selected, Some(Point::new(mouse_pos.0, mouse_pos.1)));
        }
    } else {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, invalid_bg);
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }
    (ItemMenuResult::NoResponse, None)
}