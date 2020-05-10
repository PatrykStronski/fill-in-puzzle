use crate::puzzle::Puzzle;

fn find_index(lexicone: &Vec<String>, word:String) -> i16 {
    for i in 0..lexicone.len() {
        if lexicone[i] == word {
            return i as i16;
        }
    }
    return -1;
}

fn backtrack_step(puz: &mut Puzzle, lexicone: &mut Vec<String>, x: usize, y: usize) -> bool {
    if puz.is_hash(x, y) {
        return backtrack_step(puz, lexicone, x + 1, y);
    }
    if puz.exceed_bonds(x, y) {
        if puz.y_critical(y) {
            println!("{}", puz.print_current_board());
            return puz.validate_puzzle();
        }
        return backtrack_step(puz,lexicone, 0, y+1);
    }
    let mut lex_len = lexicone.len();
    let mut index: usize = 0;
    while index < lex_len {
        let word = lexicone[index].to_string();
        if puz.write_word(word.to_string(), x, y, true) {
            let to_remove = find_index(lexicone, word.to_string());
            if to_remove == -1 {
                continue;
            }
            lexicone.remove(to_remove as usize);
            lex_len -= 1;
            let mut x_shifted = x + word.len();
            if puz.get_element(x_shifted, y) == '#' {
                x_shifted += 1;
            }
            if x_shifted >= puz.width {
                if backtrack_step(puz, &mut lexicone.to_vec(), 0, y+1) {
                    return true;
                }
            }
            if backtrack_step(puz, &mut lexicone.to_vec(), x_shifted, y) {
                return true;
            }
        }
        index += 1;
    }
    return false;
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