use tetra::input::Key;

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn key_to_dir(key: &Key) -> Option<MoveDirection> {
    match key {
        Key::W => Some(MoveDirection::Up),
        Key::S => Some(MoveDirection::Down),
        Key::A => Some(MoveDirection::Left),
        Key::D => Some(MoveDirection::Right),
        _ => None,
    }
}
