use crate::puzzle::{Puzzle, Variable};
use std::time::Instant;

pub fn delete_from_forthcoming_domains(
    domains: &Vec<Variable>,
    word: String,
    index: usize,
) -> Vec<Variable> {
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

fn remove_word_from_lexicone(lexicone: &Vec<String>, word: &String) -> Vec<String> {
    let mut curr_lexicone = lexicone.to_vec();
    curr_lexicone.retain(|x| *x != *word);
    return curr_lexicone;
}

fn backtrack_step(puz: &mut Puzzle, domains: Vec<Variable>, lexicone: &mut Vec<String>, index: usize) -> bool {
    if index >= domains.len() {
        return puz.validate_horizontals(lexicone);
    }
    let var = domains[index].clone();
    for word in var.domain.to_vec() {
        let start = var.starting_index;
        puz.write_word(word.to_string(), start);
        let mut new_lexicone = remove_word_from_lexicone(lexicone, &word);
        let new_domains = delete_from_forthcoming_domains(&domains, word, index);
        if backtrack_step(puz, new_domains, &mut new_lexicone, index + 1) {
            return true;
        }
    }
    return false;
}

fn backtrack(puz: &mut Puzzle, domains: Vec<Variable>) -> bool {
    return backtrack_step(puz, domains.to_vec(), &mut puz.lexicone.to_vec(), 0);
}

pub fn solve(puz: &mut Puzzle, nmd: usize) {
    let variable_board = puz.create_variable_board();
    let now = Instant::now();

    if backtrack(puz, variable_board) {
        println!("SOLVED");
        println!("{}", puz.print_current_board());
        println!(
            "Execution time with backtracking: {}",
            now.elapsed().as_millis()
        );
    } else {
        println!("FAILED");
        println!("{}", puz.print_current_board());
    }
}
