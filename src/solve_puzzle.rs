use crate::puzzle::Puzzle;

fn remove_from_lexicone(lexicone: &mut Vec<String>, word:String) -> bool {
    for i in 0..lexicone.len() {
        if lexicone[i] == word {
            lexicone.remove(i);
            return true;
        }
    }
    return false;
}

fn backtrack_step(puz: &mut Puzzle, lexicone: &mut Vec<String>, x: usize, y: usize) -> bool {
    if puz.is_hash(x, y) {
        return backtrack_step(puz, lexicone, x + 1, y);
    }
    if puz.exceed_bonds(x, y) {
        if puz.y_critical(y) {
            return puz.validate_puzzle();
        }
        return backtrack_step(puz,lexicone, 0, y+1)
    }
    for word in lexicone.to_vec() {
        if puz.write_word(word.to_string(), x, y, true) {
            if !remove_from_lexicone(lexicone, word.to_string()) {
                continue;
            }
            let mut x_shifted = x + word.len();
            if puz.get_element(x_shifted, y) == '#' {
                x_shifted += 1;
            }
            if x_shifted >= puz.width {
                return backtrack_step(puz, &mut lexicone.to_vec(), 0, y+1);
            }
            return backtrack_step(puz, &mut lexicone.to_vec(), x_shifted, y);
        }
    }
    return puz.validate_puzzle();
}

fn backtrack(puz: &mut Puzzle) -> bool {
    return backtrack_step(puz, &mut puz.lexicone.to_vec(), 0,0);
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