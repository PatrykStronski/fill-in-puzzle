pub struct Puzzle {
    pub init_board: Vec<char>,
    pub current_board: Vec<char>,
    pub width: usize,
    pub height: usize,
    pub lexicone: Vec<String>
}

impl Puzzle {
    fn calculate_index(&self, pos_x: usize, pos_y: usize) -> usize {
        return pos_y * self.width + pos_x;
    }

    pub fn exceed_bonds(&self, pos_x: usize, pos_y: usize) -> bool {
        return self.calculate_index(pos_x, pos_y) >= (self.width * self.height);
    }

    fn calculate_eof(&self, ind: usize) -> usize {
        return (( ind / self.width ) +1 ) * self.width -1;
    }

    pub fn get_element(&self, pos_x: usize, pos_y: usize) -> char {
        if self.exceed_bonds(pos_x, pos_y) {
            return '\0';
        }
        return self.current_board[self.calculate_index(pos_x, pos_y)];
    }

    pub fn print_current_board(&self) -> String {
        let mut board = "".to_string();
        for y in 0..self.height {
            for x in 0..self.width {
                board.push_str(&format!("{}",self.get_element(x, y)));
            }
            board.push_str("\n");
        }
        return board;
    }

    fn get_horizontal_word(&self, pos_x: usize, pos_y: usize) -> String {
        let mut index = self.calculate_index(pos_x, pos_y);
        let max = self.width * self.height;
        let mut word = String::new();
        while index < max {
            if self.current_board[index] == '#' {
                break;
            }
            word.push(self.current_board[index]);
            index += self.width;
        }
        if word.len() < 2 {
            return String::from("");
        }
        return word;
    }

    fn get_veritcal_word(&self, pos_x: usize, pos_y: usize) -> String {
        let index = self.calculate_index(pos_x, pos_y);
        let eof = self.calculate_eof(index);
        let mut word = String::new();
        for i in index..eof {
            if self.current_board[i] == '#' {
                break;
            }
            word.push(self.current_board[i]);
        }
        if word.len() < 2 {
            return String::from("");
        }
        return word;
    }

    fn get_horizontal_word_vec(&self, mut index: usize) -> String {
        let max = self.width * self.height;
        let mut word = String::new();
        while index < max {
            if self.current_board[index] == '#' {
                break;
            }
            word.push(self.current_board[index]);
            index += self.width;
        }
        if word.len() < 2 {
            return String::from("");
        }
        return word;
    }

    fn get_veritcal_word_vec(&self, index: usize) -> String {
        let eof = self.calculate_eof(index);
        let mut word = String::new();
        for i in index..(eof+1) {
            if self.current_board[i] == '#' {
                break;
            }
            word.push(self.current_board[i]);
        }
        if word.len() < 2 {
            return String::from("");
        }
        return word;
    }

    pub fn get_word(&self, pos_x: usize, pos_y: usize, vertical: bool) -> String {
        if vertical {
            return self.get_veritcal_word(pos_x, pos_y);
        }
        return self.get_horizontal_word(pos_x, pos_y);
    }

    fn write_horizontal_word(&mut self, word: String, pos_x: usize, pos_y: usize) -> bool {
        let mut index = self.calculate_index(pos_x, pos_y);
        let max = self.width * self.height;
        for c in word.chars() {
            if index >= max {
                return false;
            }
            if self.current_board[index] == '#' {
                return false;
            }
            self.current_board[index] = c;
            index += self.width;
        }
        return true;
    }

    fn write_veritcal_word(&mut self, word: String, pos_x: usize, pos_y: usize) -> bool {
        let mut index = self.calculate_index(pos_x, pos_y);
        let eof = self.calculate_eof(index);
        for c in word.chars() {
            if index > eof {
                return false;
            }
            if self.current_board[index] == '#' {
                return false;
            }
            self.current_board[index] = c;
            index += 1;
        }
        return true;
    }

    pub fn write_word(&mut self, word: String, pos_x: usize, pos_y: usize, vertical: bool) -> bool {
        if vertical {
            return self.write_veritcal_word(word, pos_x, pos_y);
        }
        return self.write_horizontal_word(word, pos_x, pos_y);
    }

    fn get_word_index_from_lexicone(&self, curr_lexicone: &Vec<String>, word: String) -> i16 {
        for x in 0..curr_lexicone.len() {
            if word == curr_lexicone[x] {
                return x as i16;
            }
        }
        return -1;
    }

    fn remove_word_from_lexicone(&self, curr_lexicone: &mut Vec<String>, word: String) -> bool {
        for x in 0..curr_lexicone.len() {
            if word == curr_lexicone[x] {
                curr_lexicone.remove(x);
                return true;
            }
        }
        return false;
    }

    fn validate_verticals(&self, curr_lexicone: &mut Vec<String>) -> bool {
        let mut ind: usize = 0;
        let max = self.current_board.len();
        while ind < max {
            let word = self.get_veritcal_word_vec(ind).to_string();
            if word == "" {
                ind += 1;
                continue;
            }
            if self.remove_word_from_lexicone(curr_lexicone, word.to_string()) {
                ind += word.len();
            } else {
                return false;
            }
        }
        return true;
    }

    fn validate_horizontals(&self, curr_lexicone: &mut Vec<String>) -> bool {
        let max = self.height;
        for x in 0..self.width {
            let mut y = 0;
            while y < max {
                let word = self.get_horizontal_word(x, y).to_string();
                if self.remove_word_from_lexicone(curr_lexicone, word.to_string()) {
                    y += word.len();
                } else {
                    if word.len() < 1 {
                        y += 1;
                        continue;
                    }
                    return false;
                }
            }
        }
        return true;
    }

    pub fn validate_puzzle(&self) -> bool {
        let mut curr_lexicone = self.lexicone.to_vec();
        let vertical = self.validate_verticals(&mut curr_lexicone);
        let horizontal = self.validate_horizontals(&mut curr_lexicone);
        return vertical && horizontal;
    }
}