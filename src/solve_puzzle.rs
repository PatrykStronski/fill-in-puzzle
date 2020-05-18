use crate::puzzle::Puzzle;
use crate::wordlengths::WordLengths;

fn backtrack_step(puz: &mut Puzzle, index: usize) -> bool {
    if index >= puz.variable_board.len() {
        return puz.validate_puzzle();
    }
    let var = puz.variable_board[index].clone();
    for word in var.domain.to_vec() {
        let start = var.starting_index;
        puz.write_word(word, start);
        if backtrack_step(puz, index + 1) {
            return true;
        }
    }
    return false;
}

fn backtrack(puz: &mut Puzzle) -> bool {
    return backtrack_step(puz, 0);
}

pub fn solve(puz: &mut Puzzle, nmd: usize) {
    puz.create_variable_board();
    if backtrack(puz) {
        println!("SOLVED");
        println!("{}", puz.print_current_board());
    } else {
        println!("FAILED");
        println!("{}", puz.print_current_board());
    }
}
