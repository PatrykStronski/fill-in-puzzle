mod load_puzzle;
mod puzzle;
use std::io::Error;

fn main() -> Result<(), Error> {
    let max_puzzle = 5;
    let mut puzzles = Vec::<puzzle::
    Puzzle>::new();
    for puz_number in 0..max_puzzle {
        let new_puzzle = load_puzzle::get_puzzle(puz_number)?;
        println!("{}", new_puzzle.print_current_board());
        puzzles.push(new_puzzle);
    }
    Ok(())
}
