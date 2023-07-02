use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod colors;
use colors::*;
mod components;
pub use components::*;
mod glyph_index;
use glyph_index::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
mod visibility_system;
use visibility_system::VisibilitySystem;


pub struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

rltk::embedded_resource!(GAME_FONT,"../resources/cp437_16x16.png");

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    rltk::link_resource!(GAME_FONT, "resources/cp437_16x16.png");
    
    let mut context = RltkBuilder::simple(80, 50)
        .unwrap()
        .with_title("McGufffin Quest")
        .with_font("cp437_16x16.png", 16, 16)
        .with_tile_dimensions(16, 16)
        .with_fps_cap(30.0)
        .build()?;

    context.set_active_font(1, true);

    context.screen_burn_color(RGB::named(SCREENBURN_COLOR));
    context.with_post_scanlines(true);

    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let(player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

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
        .build();

    rltk::main_loop(context, gs)
}
