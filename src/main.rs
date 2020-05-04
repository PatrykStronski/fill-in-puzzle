mod load_puzzle;
mod puzzle;

fn main() -> Result<(), ()> {
    let max_puzzle = 5;
    let mut puzzles = Vec::<puzzle::
    Puzzle>::new();
    for puz_number in 0..max_puzzle {
        let new_puzzle = load_puzzle::get_puzzle(puz_number)?;
        println!("{:?}", new_puzzle);
        puzzles.push(new_puzzle);
    }
    Ok(())
}
