use crate::{internal::field_manager::PlayfieldManager, models::playfield::Playfield};

#[test]
fn test_get_cells_around() {
    let field_size = 100;
    let field = Playfield::create(field_size);
    let manager = PlayfieldManager {};
    let cells_around_for_corner_cell_size = 3;
    let left_top_corner = manager.get_cells_around(0, 0, &field);
    assert_eq!(left_top_corner.len(), cells_around_for_corner_cell_size);
    let right_top_corner = manager.get_cells_around(0, field_size, &field);
    assert_eq!(right_top_corner.len(), cells_around_for_corner_cell_size);
    let left_bottom_corner = manager.get_cells_around(field_size, 0, &field);
    assert_eq!(left_bottom_corner.len(), cells_around_for_corner_cell_size);
    let right_bottom_corner = manager.get_cells_around(field_size, field_size, &field);
    assert_eq!(right_bottom_corner.len(), cells_around_for_corner_cell_size);

    let any_cell = manager.get_cells_around(10, 10, &field);
    assert_eq!(any_cell.len(), 8);
}
