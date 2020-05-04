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

    fn calculate_eof(&self, ind: usize) -> usize {
        return (( ind / self.width ) +1 ) * self.width -1;
    }

    fn get_element(&self, pos_x: usize, pos_y: usize) -> char {
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

    fn write_word(&mut self, word: String, pos_x: usize, pos_y: usize, vertical: bool) -> bool {
        if vertical {
            return self.write_veritcal_word(word, pos_x, pos_y);
        }
        return self.write_horizontal_word(word, pos_x, pos_y);
    }
}