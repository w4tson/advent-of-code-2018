use std::collections::HashMap;

struct Game {
    players: u32,
    marbles: u32,
    board: Vec<u32>,
    curr_index: usize,
    curr_player: u32,
    score: HashMap<u32, u32>
}

impl Game {
    pub fn new(players: u32, marbles: u32) -> Game {
        Game { curr_player: 1, curr_index: 0, marbles, players, board: vec![0], score: HashMap::new() }
    }
    
    pub fn highest_score(&self) -> u32 {
        self.score.iter()
            .map(|(_, &v)| v)
            .max()
            .unwrap_or_default()
    }
    
    pub fn turn(&mut self, m: u32) {
        
        if m % 23 == 0 {
        
            let index_to_remove = Game::index_to_remove(self.board.len(), self.curr_index);
            
            let score = self.board.remove(index_to_remove);
            
            println!("Score {}   m ({})", score, m);
            *self.score.entry(self.curr_player)
                .or_default() += score + m;

            self.curr_index = index_to_remove;


        } else {
            if self.board.len() < 3 {
                &self.board.insert(1, m);
                self.curr_index = 1;
            } else {
                let index = match self.board.len() - self.curr_index  {
                    1 => 1,
                    _ => self.curr_index + 2
                };

              
//                println!("curr index = {}   next {}", self.curr_index, index);
                self.board.insert(index, m);
                self.curr_index = index;
                
            }
        }
        
//        self.print_board();


        self.curr_player = if self.curr_player < self.players { self.curr_player + 1 } else { 1 };
    }

    fn print_board(&self) {
        
        print!("[{}] ", self.curr_player );
        for (i, m) in self.board.iter().enumerate() {
            
            match i == self.curr_index {
                true => print!("({})", m),
                _ => print!(" {} ", m)
            } 
            
            
        }
        println!();
    }
    
    pub fn index_to_remove(data_len: usize, curr_index: usize) -> usize {
        match curr_index < 7 {
            true => {
                data_len - (7 - curr_index)
            },
            _ => {
                curr_index - 7
            }
        }
    }
}



fn play_game(players: u32, marbles: u32) -> u32 {
    let mut game = Game::new(players, marbles);
    println!("[~] (0)");
    for m in 1..marbles+1 {
        
        game.turn(m);
    } 

    game.highest_score()
}


mod tests {
    use spectral::assert_that;
    use super::*;
    use crate::utils::file::read_puzzle_input;

    #[test]
    fn test() {
        
        assert_eq!(0, Game::index_to_remove(10, 7));
        assert_eq!(9, Game::index_to_remove(10, 6));
        assert_eq!(8, Game::index_to_remove(10, 5));
        assert_eq!(7, Game::index_to_remove(10, 4));
        assert_eq!(6, Game::index_to_remove(10, 3));
        assert_eq!(5, Game::index_to_remove(10, 2));
        assert_eq!(4, Game::index_to_remove(10, 1));
        assert_eq!(3, Game::index_to_remove(10, 0));
        
        
        assert_eq!(32, play_game(9, 25));
        assert_eq!(8317, play_game(10, 1618));
        assert_eq!(146373, play_game(13, 7999));
        assert_eq!(2764, play_game(17, 1104));
        assert_eq!(54718, play_game(21, 6111));
        assert_eq!(37305, play_game(30, 5807));
        let result = play_game(411, 71170);
        println!("{}", result);
    }
    
    #[test]
    fn another() {
        
    }
    

}
