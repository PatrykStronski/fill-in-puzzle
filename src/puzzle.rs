pub struct Puzzle {
    init_board: Vec<char>,
    current_board: Vec<char>,
    width: usize,
    height: usize,
    lexicone: Vec<String>
}

impl Puzzle {
    fn calculate_index(&self, pos_x: usize, pos_y: usize) -> usize {
        return pos_y * self.width + pos_x;
    }

    fn get_element(&self, pos_x: usize, pos_y: usize) {
        return self.current_board[self.calculate_index(pos_x, pos_y)];
    }

    pub fn print_current_board(&self) -> String {
        let board = "";
        for y in 0..self.height {
            for x in 0..self.width {
                board.push(self.get_element(x, y));
            }
            board.push('\n');
        }
        return board;
    }
}