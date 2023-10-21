use std::collections::HashMap;

use crate::models::{cell::Square, position::Position};

pub fn build_beacon_pattern() -> HashMap<Position, Square> {
    let mut result = HashMap::new();
    result.insert(Position::new(0, 0), Square::new(Some(true)));
    result.insert(Position::new(0, 1), Square::new(Some(true)));
    result.insert(Position::new(1, 0), Square::new(Some(true)));
    result.insert(Position::new(1, 1), Square::new(Some(true)));
    result.insert(Position::new(2, 2), Square::new(Some(true)));
    result.insert(Position::new(2, 3), Square::new(Some(true)));
    result.insert(Position::new(3, 2), Square::new(Some(true)));
    result.insert(Position::new(3, 3), Square::new(Some(true)));
    return result;
}

#[allow(dead_code)]
pub fn build_blinker_pattern() -> HashMap<Position, Square> {
    let mut result = HashMap::new();
    result.insert(Position::new(10, 10), Square::new(Some(true)));
    result.insert(Position::new(10, 11), Square::new(Some(true)));
    result.insert(Position::new(10, 12), Square::new(Some(true)));
    return result;
}
