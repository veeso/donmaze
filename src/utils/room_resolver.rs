//! # Room resolver
//!
//! This utils module provides function to resolve directions and room type from edges

use crate::game::Session;
use crate::gfx::Room as RenderRoom;

/// Get direction for room.
/// Direction is JUST A VIRTUAL CONCEPT, which is resolved with the following rules:
///
/// 0. IF the current room has 1 edge, direction is always AHEAD (NOTE: won't be displayed by session)
/// 1. IF the current room has 2 edges, AND the player room IS NOT 0, the room direction is always AHEAD
/// 2. IF the current room has 2 edges, AND the previous room IS UNSET, the room direction is case 3
/// 3. IF the current room has 3 edges, the room direction is
///   1. RIGHT IF `room` > all the other edges
///   2. ELSE LEFT
/// 4. ELSE GIVEN the sorted edges, apply the following rule
///    1. the BIGGEST or the LOWEST is AHEAD
///    2. the second bigger is RIGHT
///    3. the third bigger is LEFT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Ahead,
    Right,
    Left,
}

/// Resolve edges to room to render.
///
/// Panics if params are invalid
pub fn resolve_room_to_render(edges: usize, previous_room_set: bool, is_exit: bool) -> RenderRoom {
    match (edges, previous_room_set, is_exit) {
        (1, true, false) => RenderRoom::DeadEnd,
        (1, true, true) => RenderRoom::DeadEndWithMazeExit,
        (1, false, false) => RenderRoom::Corridor,
        (1, false, true) => RenderRoom::CorridorWithMazeExit,
        (2, true, false) => RenderRoom::Corridor,
        (2, true, true) => RenderRoom::CorridorWithMazeExit,
        (3, false, false) => RenderRoom::ThreeExit,
        (2 | 3, _, false) => RenderRoom::TwoExit,
        (2 | 3, _, true) => RenderRoom::TwoExitWithMazeExit,
        (4, _, _) => RenderRoom::ThreeExit,
        _ => panic!("unable to resolve room render"),
    }
}

/// Resolve room direction according to the direction rules
pub fn resolve_room_direction(room: u32, session: &Session) -> Direction {
    let mut room_edges: Vec<u32> = session
        .adjacent_rooms()
        .into_iter()
        .filter(|x| Some(*x) != session.get_last_room())
        .collect();
    room_edges.sort();
    // room edges DOES NOT CONTAIN PREVIOUS ROOM AT THIS POINT
    if room_edges.len() <= 1 {
        Direction::Ahead
    } else if room_edges.len() == 2 {
        if room_edges[1] == room {
            Direction::Right
        } else {
            Direction::Left
        }
    } else if room_edges.len() == 3 {
        if room_edges[2] == room {
            Direction::Ahead
        } else if room_edges[1] == room {
            Direction::Right
        } else {
            Direction::Left
        }
    } else {
        panic!(
            "could not resolve room direction. Edges {:?}, room: {}",
            room_edges, room
        );
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_resolve_room_to_render() {
        assert_eq!(resolve_room_to_render(1, true, false), RenderRoom::DeadEnd);
        assert_eq!(
            resolve_room_to_render(1, true, true),
            RenderRoom::DeadEndWithMazeExit
        );
        assert_eq!(
            resolve_room_to_render(1, false, false),
            RenderRoom::Corridor
        );
        assert_eq!(
            resolve_room_to_render(1, false, true),
            RenderRoom::CorridorWithMazeExit
        );
        assert_eq!(resolve_room_to_render(2, false, false), RenderRoom::TwoExit);
        assert_eq!(
            resolve_room_to_render(2, false, true),
            RenderRoom::TwoExitWithMazeExit
        );
        assert_eq!(
            resolve_room_to_render(3, false, false),
            RenderRoom::ThreeExit
        );
        assert_eq!(resolve_room_to_render(3, true, false), RenderRoom::TwoExit);
        assert_eq!(
            resolve_room_to_render(4, false, false),
            RenderRoom::ThreeExit
        );
    }

    #[test]
    #[should_panic]
    fn should_fail_resolving_room_to_render() {
        resolve_room_to_render(5, true, true);
    }

    #[test]
    fn should_resolve_room_direction() {
        let mut session = Session::mock();
        // two edges, but previous room is unset
        assert_eq!(resolve_room_direction(1, &session), Direction::Left);
        assert_eq!(resolve_room_direction(2, &session), Direction::Right);
        // two edges, but previous room is SET
        session.maze.player = 8;
        session.set_last_room(7);
        assert_eq!(resolve_room_direction(7, &session), Direction::Ahead);
        // one edge
        session.maze.player = 9;
        assert_eq!(resolve_room_direction(8, &session), Direction::Ahead);
        // three edges
        session.maze.player = 1;
        session.set_last_room(0);
        assert_eq!(resolve_room_direction(9, &session), Direction::Right);
        assert_eq!(resolve_room_direction(3, &session), Direction::Left);
        // four edges
        session.maze.player = 4;
        session.set_last_room(2);
        assert_eq!(resolve_room_direction(7, &session), Direction::Ahead);
        assert_eq!(resolve_room_direction(6, &session), Direction::Right);
        assert_eq!(resolve_room_direction(5, &session), Direction::Left);
    }
}
