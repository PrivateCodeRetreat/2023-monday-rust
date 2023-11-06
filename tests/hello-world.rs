#![cfg_attr(feature = "strict", deny(warnings))]

use hello_world::Board;
#[test]
fn die_from_underpopulation() {
    let iterations = 1;

    let mut board = Board::withAliveCellsAt(vec![(1, 1)]);
    let mut storyboard = board.to_string();
    for _ in 0..iterations {
        board = board.next_state();
        storyboard = format!("{}\n\n{}", storyboard, board);
    }
    
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
"#
    .trim_start();
    if (storyboard != expected_storyboard) {
        println!("storyboard:\n{}", storyboard);
    }
    assert_eq!(storyboard, expected_storyboard);
}
