use crate::puzzle::Puzzle;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct Board {
    height: usize,
    width: usize,
    board: Vec<char>
}

fn get_board(filename: String) -> Result<Board, Error> {
    let file = File::open(filename)?;
    let content = BufReader::new(file);

    let lines = content.lines();
    let hght = lines.len()?;
    let wdth = lines[0].len()?;

    let mut brd = Vec::<char>::new();
    for line in lines {
        for c in line.chars() {
            brd.push(c);
        }
    }
    Ok(Board {
        width: wdth,
        height: hght,
        board: brd
    })
}

fn get_lexicon(filename: String) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    let content = BufReader::new(file);
    let mut lexicon = Vec::<String>::new();
    for line in content.lines() {
        lexicon.push(line?);
    }
    Ok(lexicon)
}

pub fn get_puzzle(number: usize) -> Result<Puzzle, Error> {
    let puz_name = format!("./resources/puzzle{}", number)?;
    let lexicon_name = format!("words{}", number)?;

    let board = get_board(puz_name)?;
    let lexicon = get_lexicon(lexicon_name)?;

    let puz_full = Puzzle {
        init_board: board.board.to_vec(),
        current_board: board.board.to_vec(),
        width: board.width,
        height: board.height,
        lexicone: lexicon.to_vec(),
    };
    Ok(puz_full)
}
