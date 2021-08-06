use bracket_lib::prelude::*;
use specs::prelude::*;
use specs::Component;
use std::cmp::{min, max};

use crate::{
    components::Position,
    map::{Map, TileType},
    RunState,
    State,
};

#[derive(Component, Debug)]
pub struct Player {}

/// Tries to move the player. Will not move player if target position is blocked.
pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = map.xy_idx(pos.x + dx, pos.y + dy);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => {
            return RunState::Paused;
        },
        Some(key) => match key {
            // fullscreen toggle
            VirtualKeyCode::Return => {
                if ctx.alt {
         
                }
            },

            VirtualKeyCode::Left
            | VirtualKeyCode::Numpad4
            | VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right
            | VirtualKeyCode::Numpad6
            | VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up
            | VirtualKeyCode::Numpad8
            | VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down
            | VirtualKeyCode::Numpad2
            | VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            _ => {
                return RunState::Paused;
            }
        }
    }

    RunState::Running
}