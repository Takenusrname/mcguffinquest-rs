use rltk::{ Point, RGB, Rltk};
use specs::prelude::*;

use rltk::Rect;
use super::colors::*;
use super::{ game_log::GameLog, CombatStats, Map, Name, Player, Position };

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