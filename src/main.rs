mod load_puzzle;
mod puzzle;
mod solve_puzzle;
mod wordlengths;
use std::io::Error;

fn main() -> Result<(), Error> {
    let max_puzzle = 5;
    for puz_number in 0..max_puzzle {
        let mut new_puzzle = load_puzzle::get_puzzle(puz_number)?;
        solve_puzzle::solve(&mut new_puzzle, puz_number);
    }
    Ok(())
}
