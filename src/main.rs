use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

mod colors;
use colors::*;
mod damage_system;
use damage_system::DamageSystem;
mod components;
pub use components::*;
mod game_log;
mod glyph_index;
use glyph_index::*;
mod gui;
mod map;
pub use map::*;
mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
use melee_combat_system::MeleeCombatSystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod player;
use player::*;
mod rect;
mod visibility_system;
use visibility_system::VisibilitySystem;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { AwaitingInput, PreRun, PlayerTurn, MonsterTurn }


pub struct State {
    pub ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        damage_system::delete_the_dead(&mut self.ecs);

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

        gui::draw_ui(&self.ecs, ctx);
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem{};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem{};
        damage.run_now(&self.ecs);
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
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let(player_x, player_y) = map.rooms[0].center();

    let player_entity = gs.ecs
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
        //.with(BlocksTile{})
        .with(CombatStats{max_hp: 30, hp: 30, defense: 2, power: 5})
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
            .with(BlocksTile{})
            .with(CombatStats{max_hp: 16, hp: 16, defense: 1, power: 4})
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(game_log::GameLog{ entries: vec!["Welcome to McGuffin Quest".to_string()]});

    rltk::main_loop(context, gs)
}
