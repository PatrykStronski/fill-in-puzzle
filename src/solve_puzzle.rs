use crate::puzzle::Puzzle;

fn backtrack_step(puz: &mut Puzzle, x: usize, y: usize) -> bool {
    println!("x: {} y: {}", x, y);
    if puz.exceed_bonds(x, y) {
        return puz.validate_puzzle();
    }
    for word in puz.lexicone.to_vec() {
        if puz.write_word(word.to_string(), x, y, true) {
            let mut x_shifted = x + word.len();
            if puz.get_element(x_shifted, y) == '#' {
                x_shifted += 1;
            }
            if x_shifted >= puz.width {
                return backtrack_step(puz, 0, y+1);
            }
            return backtrack_step(puz, x_shifted, y);
        }
    }
    return puz.validate_puzzle();
}

fn backtrack(puz: &mut Puzzle) -> bool {
    return backtrack_step(puz, 0,0);
}

pub fn solve(puz: &mut Puzzle, nmd: usize) {
    if backtrack(puz) {
        println!("SOLVED");
        println!("{}", puz.print_current_board());
    } else {
        println!("FAILED");
        println!("{}", puz.print_current_board());
    }
}