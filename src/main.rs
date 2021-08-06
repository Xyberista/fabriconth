use bracket_lib::prelude::*;

mod game;
pub use game::State;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Fabriconth")
        .with_fps_cap(60.0)
        .with_fullscreen(true)
        .build()?;
        
    let gamestate = State {

    };
    
    main_loop(context, gamestate)
}
