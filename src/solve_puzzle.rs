use crate::puzzle::{Puzzle, Variable};

pub fn delete_from_forthcoming_domains(domains: &Vec<Variable>, word: String, index: usize) -> Vec<Variable> {
    let mut new_domains = domains.to_vec();
    for i in index..new_domains.len() {
        let mut var = &mut new_domains[i];
        if var.length != word.len() {
            continue;
        }
        var.domain.retain(|x| *x != word);
    }
    return new_domains;
}

fn backtrack_step(puz: &mut Puzzle, domains: Vec<Variable>, index: usize) -> bool {
    if index >= puz.variable_board.len() {
        return puz.validate_puzzle();
    }
    let var = puz.variable_board[index].clone();
    for word in var.domain.to_vec() {
        let start = var.starting_index;
        puz.write_word(word.to_string(), start);
        let new_domains = delete_from_forthcoming_domains(&domains ,word.to_string(), index);
        if backtrack_step(puz, new_domains, index + 1) {
            return true;
        }
    }
    return false;
}

fn backtrack(puz: &mut Puzzle) -> bool {
    return backtrack_step(puz, puz.variable_board.to_vec(), 0);
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
