use crate::puzzle::Puzzle;
use crate::wordlengths::WordLengths;

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

fn filter_lexicone_by_length(lexicone: &Vec<String>, length: usize) -> Vec<String> {
    let mut new_lexicone = Vec::<String>::new();
    for word in lexicone {
        if word.len() == length {
            new_lexicone.push(word.to_string());
        }
    }
    return new_lexicone;
}

fn backtrack_step(puz: &mut Puzzle, lexicone: &mut Vec<String>, x: usize, y: usize) -> bool {
    //println!("x: {}, y: {}", x, y);
    //println!("{}", puz.print_current_board());
    if puz.exceed_bonds(x, y) {
        return puz.validate_puzzle();
    }
    if puz.x_outof_range(x) {
        return backtrack_step(puz, lexicone, 0, y + 1);
    }
    if puz.is_hash(x, y) {
        return backtrack_step(puz, lexicone, x + 1, y);
    }
    let word_len = puz.get_vertical_word_length(x, y);
    let lexicone_here = filter_lexicone_by_length(lexicone, word_len);
    for word in lexicone_here {
        if puz.write_word(word.to_string(), x, y) {
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
