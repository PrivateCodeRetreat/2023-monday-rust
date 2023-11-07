#![cfg_attr(feature = "strict", deny(warnings))]

use hello_world::Board;
#[test]
fn die_from_underpopulation() {
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
"#;

    verify_game_of_life(vec![(1, 1)], 1, expected_storyboard);
}

#[test]
fn blinker() {
    let expected_storyboard = r#"
..........
..***.....
.***......
..........
..........


...*......
.*..*.....
.*..*.....
..*.......
..........


..........
..***.....
.***......
..........
..........


...*......
.*..*.....
.*..*.....
..*.......
..........
"#;

    verify_game_of_life(vec![(2,1), (3,1), (4,1), (1,2), (2,2), (3,2)], 3, expected_storyboard);
}

#[test]
fn traveller() {
    let expected_storyboard = r#"
..........
..*.......
...*......
.***......
..........


..........
..........
.*.*......
..**......
..*.......


..........
..........
...*......
.*.*......
..**......


..........
..........
..*.......
...**.....
..**......


..........
..........
...*......
....*.....
..***.....
"#;
    verify_game_of_life(vec![(2, 1), (3, 2), (1, 3), (2, 3), (3, 3)], 4, expected_storyboard);

}

fn verify_game_of_life(alive_cells: Vec<(i32, i32)>, iterations: i32, mut expected_storyboard: &str) {
    let mut board = Board::with_alive_cells_at(alive_cells);
    let mut storyboard = board.to_string();
    for _ in 0..iterations {
        board = board.next_state();
        storyboard = format!("{}\n\n{}", storyboard, board);
    }

    
    expected_storyboard = expected_storyboard.trim_start();
    if storyboard != expected_storyboard {
        println!("storyboard:\n{}", storyboard);
    }
    assert_eq!(storyboard, expected_storyboard);
}
