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
}