use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::{components::{Position, Renderable}, map::{draw_map, Map}, player::*};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
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
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}