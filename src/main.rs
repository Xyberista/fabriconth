use bracket_lib::prelude::*;
use components::{Name, Position, Renderable};
use map::Map;
use player::Player;
use specs::prelude::*;

mod game;
pub use game::*;
mod components;
mod player;
mod map;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Fabriconth")
        .with_fps_cap(60.0)
        .build()?;
        
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Name>();
    
    let map: Map = Map::new_base_map();
    let (player_x, player_y) = (0, 0);

    gs.ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .with(Name {name: "Player".to_string() })
        .build();
    
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    main_loop(context, gs)
}
