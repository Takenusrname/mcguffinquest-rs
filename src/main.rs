use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

mod colors;
use colors::*;
mod components;
pub use components::*;
mod glyph_index;
use glyph_index::*;
mod map;
pub use map::*;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod player;
use player::*;
mod rect;
mod visibility_system;
use visibility_system::VisibilitySystem;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }


pub struct State {
    pub ecs: World,
    pub runstate: RunState
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

rltk::embedded_resource!(GAME_FONT, "../resources/cp437_16x16_mod.png");
rltk::embedded_resource!(GAME_FONT2,"../resources/cp437_16x16.png");

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    rltk::link_resource!(GAME_FONT, "resources/cp437_16x16_mod.png");
    rltk::link_resource!(GAME_FONT2, "resources/cp437_16x16.png");
    
    let mut context = RltkBuilder::simple(80, 50)
        .unwrap()
        .with_title("McGufffin Quest")
        .with_font("cp437_16x16_mod.png", 16, 16)
        .with_font("cp437_16x16.png", 16, 16)
        .with_tile_dimensions(16, 16)
        .with_fps_cap(30.0)
        .build()?;

    context.set_active_font(1, true);

    context.screen_burn_color(RGB::named(SCREENBURN_COLOR));
    context.with_post_scanlines(true);
    context.with_mouse_visibility(false);

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let(player_x, player_y) = map.rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437(PLAYER_GLYPH),
            fg: RGB::from_f32(PLAYER_FG.0, PLAYER_FG.1, PLAYER_FG.2),
            bg: RGB::from_f32(DEFAULT_BG.0, DEFAULT_BG.1, DEFAULT_BG.2)
        })
        .with(Player {})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(Name { name: "Player".to_string() })
        .build();

    let mut rng = rltk::RandomNumberGenerator::new();
    for (i,room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;

        let roll = rng.roll_dice(1, 2);

        match roll {
            1 => { glyph = rltk::to_cp437(GOBLIN_GLYPH); name = "Goblin".to_string(); }
            _ => { glyph = rltk::to_cp437(ORC_GLYPH); name = "Orc".to_string(); }
        }

        gs.ecs.create_entity()
            .with( Position { x, y })
            .with( Renderable {
                glyph: glyph,
                fg: RGB::from_f32(ENEMY_FG.0, ENEMY_FG.1, ENEMY_FG.2),
                bg: RGB::from_f32(DEFAULT_BG.0, DEFAULT_BG.1, DEFAULT_BG.2)
            })
            .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true})
            .with(Monster{})
            .with(Name{name: format!("{} #{}", &name, i) })
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    rltk::main_loop(context, gs)
}
