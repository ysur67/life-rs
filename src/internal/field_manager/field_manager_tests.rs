use crate::{
    internal::field_manager::{manager::PlayfieldManager, naive::NaivePlayfieldManager, forloop_based_manager::ForLoopBasedPlayfieldManager},
    models::playfield::Playfield,
};

#[test]
fn test_naive_manager_get_cells_around() {
    let field_size = 100;
    let field = Playfield::create(field_size);
    let manager = NaivePlayfieldManager {};
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

#[test]
fn test_for_loop_based_manager_get_cells_around() {
    let field_size = 100;
    let field = Playfield::create(field_size);
    let manager = ForLoopBasedPlayfieldManager {};
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
