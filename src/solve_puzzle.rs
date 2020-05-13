use crate::puzzle::Puzzle;

fn get_new_lexicone(lexicone: &Vec<String>, word: &String) -> Vec<String> {
    let mut new_lexicone = lexicone.to_vec();
    for i in 0..new_lexicone.len() {
        if new_lexicone[i] == *word {
            new_lexicone.remove(i);
            return new_lexicone;
        }
    }
    return new_lexicone;
}

fn backtrack_step(puz: &mut Puzzle, lexicone: &mut Vec<String>, x: usize, y: usize) -> bool {
    // println!("x: {}, y: {}", x, y);
    if puz.exceed_bonds(x, y) {
        return puz.validate_puzzle();
    }
    if puz.x_outof_range(x) {
        return backtrack_step(puz, lexicone, 0, y + 1);
    }
    if puz.is_hash(x, y) {
        return backtrack_step(puz, lexicone, x + 1, y);
    }
    for word in lexicone.to_vec() {
        if puz.write_word(word.to_string(), x, y, true) {
            let mut lexicone_new = get_new_lexicone(lexicone, &word);
            let mut x_shifted = x + word.len();
            if backtrack_step(puz, &mut lexicone_new.to_vec(), x_shifted, y) {
                return true;
            }
        }
    }
    return false;
}

fn backtrack(puz: &mut Puzzle) -> bool {
    return backtrack_step(puz, &mut puz.lexicone.to_vec(), 0, 0);
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
