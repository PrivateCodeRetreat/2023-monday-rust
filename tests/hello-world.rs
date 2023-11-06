#![cfg_attr(feature = "strict", deny(warnings))]

use hello_world::Board;
#[test]
fn die_from_underpopulation() {
    let mut board = Board::new();
    // set cell at 1,1 alive
    board.cells.push((1, 1));
    // assert that the cell at 1,1 is alive
    assert_eq!(true, board.is_alive((1,1)));
    let mut storyboard = board.to_string();

    // call next_state()
    board = board.next_state();
    // assert that the cell at 1,1 is dead
    assert_eq!(false, board.is_alive((1,1)));

    storyboard = format!("{}\n\n{}", storyboard, board);
    print!("{}", storyboard);

    let expected_storyboard = r#"
..........
.*........
..........
..........
..........


..........
..........
..........
..........
..........
"#.trim_start();

    assert_eq!(storyboard, expected_storyboard);
}
