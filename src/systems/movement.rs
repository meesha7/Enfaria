use crate::components::{Player, Position};
use crate::data::{Map, TileKind};
use crate::input::MoveDirection;
use crate::world::GameWorld;

pub fn movement_system(world: &mut GameWorld, map: &Map, direction: MoveDirection) {
    for (_id, (mut pos, _player)) in world.ecs.query_mut::<(&mut Position, &Player)>() {
        let mut target_pos = *pos;

        use MoveDirection::*;
        match direction {
            Up => target_pos.y -= 4,
            Down => target_pos.y += 4,
            Left => target_pos.x -= 4,
            Right => target_pos.x += 4,
        }

        target_pos.x += 1;
        target_pos.y += 1;

        if let Some(tile) = map.get_tile(target_pos) {
            if tile.kind == TileKind::Blocker {
                continue;
            }
        }

        target_pos.x -= 1;
        target_pos.y -= 1;

        target_pos.x += 32;
        target_pos.y += 32;

        if let Some(tile) = map.get_tile(target_pos) {
            if tile.kind == TileKind::Blocker {
                continue;
            }

            pos.x = target_pos.x - 32;
            pos.y = target_pos.y - 32;
        }
    }
}
