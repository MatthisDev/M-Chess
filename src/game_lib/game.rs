use crate::game_lib::board::{Board, BOARD_SIZE};
use crate::game_lib::piece::{Color, PieceType};
use crate::game_lib::position::Position;
use crate::game_lib::piece::Piece;

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn init() -> Self {
        Game {
            board: Board::init(),
        }
    }
    /*
     * Waited string format:
     * - <coo1>-><coo2>
     * - coo1 or coo2: [a-h][1-8]
     */
    fn parse_move_str(move_piece: &str) -> (Position, Position) {
        let count: usize = move_piece.chars().count();

        // cannot send
        if count != 6 || &move_piece[2..=3] != "->" {
            panic!("parse_move_str: invalid send string");
        }

        let from_pos: Position = Position::from_algebraic(&move_piece[0..=1]);
        let to_pos: Position = Position::from_algebraic(&move_piece[4..=5]);

        (from_pos, to_pos)
    }
    
    fn castle_situation(&mut self, king: &Piece, to_pos: &Position) -> bool {
        
        // Vérifier si le mouvement est un roque
        let rook_positions = [
            Position::new(king.position.row, 0), // Tour côté dame
            Position::new(king.position.row, 7), // Tour côté roi
        ];

        for rook_position in rook_positions.iter() {

            if (*to_pos == Position::new(king.position.row, king.position.col - 2) ||
                *to_pos == Position::new(king.position.row, king.position.col + 2)) &&
                self.board.can_castle(&king.position, &rook_position) {

                self.board.perform_castle(&king.position, &rook_position);
                
                // #FIXME
                self.board.history.push((
                        king.position,
                        *to_pos,
                        Some(king.clone()) // EXCEPTIONNELLE SITUATION
                        //Option<Piece> de la case
                ));
                return true;
            }
        }

        return false;
    }
    
    pub fn make_move_algebraic(&mut self, moves: &str) -> Result<bool, &'static str> {
        let (from_pos, to_pos): (Position, Position) = Self::parse_move_str(moves);
        
        // get the piece and if there is not return an error
        let piece: &Piece = 
            if let Some(piece) = Piece::get_piece(&from_pos, &self.board) { piece }
            else {return Err("Invalid move: There is not piece here")};

        // rock situtation
        if piece.piece_type == PieceType::King && self.castle_situation(&piece, &to_pos){
            return Ok(true);
        }
        
        // if the piece can move + is moved 
        if self.board.move_piece(&from_pos, &to_pos) {
            
            self.board.history.push((
                    from_pos,
                    to_pos,
                    Some(piece.clone())
            ));
            
            // if the king is in check due to the move
            if self.board.is_king_in_check(self.board.turn) {
                self.undo_move();
                return Err("Le roi est toujours en échec après ce mouvement.");
            }

            println!("Success!");

            // check if there is a checkmate condition
            if self.board.is_checkmate(self.board.turn) {

                println!(
                    "Échec et mat! Le joueur {} a gagné.",
                    if self.board.turn == Color::White {
                        "Noir"
                    } else {
                        "Blanc"
                    }
                );

                // Game End
                return Ok(false);
            }

            Ok(true)
        } else {
            Err("Mouvement invalide.")
        }
    }
    
    fn undo_move(&mut self) {
        if let Some((from, to, _)) = self.board.history.pop() {
            self.board.move_piece(&to, &from);
            self.board.turn = if self.board.turn == Color::White {
                Color::Black
            } else {
                Color::White
            };
        }
    }
}
