use std::cell::RefCell;

struct Board {
  kings:   [u64; 2],
  queens:  [u64; 2],
  rooks:   [u64; 2],
  bishops: [u64; 2],
  knights: [u64; 2],
  pawns:   [u64; 2],
}

enum Side {
  White,
  Black,
}

struct CastlingAbility {
  white_kingside: bool,
  white_queenside: bool,
  black_kingside: bool,
  black_queenside: bool,
}

struct Position {
  board: Board,
  side_to_move: Side,
  castling_ability: CastlingAbility,
  halfmove_clock: u8,
  fullmove_counter: u16,
}

struct Move {
  from: u8,
  to: u8,
}

impl Position {
  fn new() -> Position {
    Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
  }

  fn from_fen(record: &str) -> Position {
    let iterator  = record.chars().peekable();
    let i = RefCell::new(iterator);
  
    let mut white_king = 0u64;
    let mut white_queens = 0u64;
    let mut white_rooks = 0u64;
    let mut white_bishops = 0u64;
    let mut white_knights = 0u64;
    let mut white_pawns = 0u64;
  
    let mut black_king = 0u64;
    let mut black_queens = 0u64;
    let mut black_rooks = 0u64;
    let mut black_bishops = 0u64;
    let mut black_knights = 0u64;
    let mut black_pawns = 0u64;
  
    let mut side_to_move = Side::White;
  
    let mut castle_white_kingside = false;
    let mut castle_white_queenside = false;
    let mut castle_black_kingside = false;
    let mut castle_black_queenside = false;
  
    let mut en_passant_target_square: u8 = 64;
  
    let mut halfmove_clock: u8 = 0;
    let mut fullmove_counter: u16 = 1;
  
    let mut parse_rank = |rank| {
      let mut square = (rank - 1) * 8;
      let boundry = rank * 8;
  
      while square < boundry {
        match i.borrow_mut().next() {
          Some(c) => match c {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
              let digit = c.to_digit(10).unwrap() as i32;
              square += digit;
            },
            'K' | 'Q' | 'R' | 'B' | 'N' | 'P' | 'k' | 'q' | 'r' | 'b' | 'n' | 'p' => {
              match c {
                'K' =>  { white_king |= 1u64 << square }
                'Q' =>  { white_queens |= 1u64 << square }
                'R' =>  { white_rooks |= 1u64 << square }
                'B' =>  { white_bishops |= 1u64 << square }
                'N' =>  { white_knights |= 1u64 << square }
                'P' =>  { white_pawns |= 1u64 << square }
                'k' =>  { black_king |= 1u64 << square }
                'q' =>  { black_queens |= 1u64 << square }
                'r' =>  { black_rooks |= 1u64 << square }
                'b' =>  { black_bishops |= 1u64 << square }
                'n' =>  { black_knights |= 1u64 << square }
                'p' =>  { black_pawns |= 1u64 << square }
                _ => { panic!("Impossible case") }
              };
              square += 1;
            }
            _ => panic!("Unexpected character in rank {} in FEN record", rank)
          },
          None => panic!("Expected well-formed rank {} in FEN record", rank)
        }
  
        assert!(square <= boundry, "Too many squares in rank {} in FEN record", rank);
      }
    };
  
    let parse_character = |character| {
      match i.borrow_mut().next() {
        Some(c) if c == character => {}
        _ => panic!("Expected '{}' in FEN record", character)
      }
    };
  
    let mut parse_piece_placement = || {
      parse_rank(8); parse_character('/');
      parse_rank(7); parse_character('/');
      parse_rank(6); parse_character('/');
      parse_rank(5); parse_character('/');
      parse_rank(4); parse_character('/');
      parse_rank(3); parse_character('/');
      parse_rank(2); parse_character('/');
      parse_rank(1);
    };
  
    let mut parse_side_to_move = || {
      match i.borrow_mut().next() {
        Some('w') => side_to_move = Side::White,
        Some('b') => side_to_move = Side::Black,
        _ => panic!("Expected 'w' or 'b' in FEN record")
      }
    };
  
    let mut parse_white_kingside = || {
      match i.borrow_mut().peek() {
        Some('K') => {
          castle_white_kingside = true
        },
        _ => {}
      }
      i.borrow_mut().next();
    };
  
    let mut parse_white_queenside = || {
      match i.borrow_mut().peek() {
        Some('Q') => {
          castle_white_queenside = true
        },
        _ => {}
      }
      i.borrow_mut().next();
    };
  
    let mut parse_black_kingside = || {
      match i.borrow_mut().peek() {
        Some('k') => {
          castle_black_kingside = true
        },
        _ => {}
      }
      i.borrow_mut().next();
    };
  
    let mut parse_black_queenside = || {
      match i.borrow_mut().peek() {
        Some('q') => {
          castle_black_queenside = true
        },
        _ => {}
      }
      i.borrow_mut().next();
    };
  
    let mut parse_castling_ability = || {
      let c = i.borrow_mut().peek().copied();
      match c {
        Some('-') => { i.borrow_mut().next(); }
        Some(_) => {
          parse_white_kingside();
          parse_white_queenside();
          parse_black_kingside();
          parse_black_queenside();
        } 
        _ => panic!("Expected well-formed castling ability (-/KQkq) in FEN record")
      }
    };
  
    let parse_square = || {
      let mut square: u8 = 0;
  
      match i.borrow_mut().next() {
        Some(file) => {
          match file {
            'a' => { square += 0 }
            'b' => { square += 1 }
            'c' => { square += 2 }
            'd' => { square += 3 }
            'e' => { square += 4 }
            'f' => { square += 5 }
            'g' => { square += 6 }
            'h' => { square += 7 }
            _ => panic!("Expected a-h for file in FEN record")
          }
        }
        None => panic!("Expected a-h for file in FEN record")
      }
  
      match i.borrow_mut().next() {
        Some(rank) => {
          match rank {
            '1' => { square += 0 * 8 }
            '2' => { square += 1 * 8 }
            '3' => { square += 2 * 8 }
            '4' => { square += 3 * 8 }
            '5' => { square += 4 * 8 }
            '6' => { square += 5 * 8 }
            '7' => { square += 6 * 8 }
            '8' => { square += 7 * 8 }
            _ => panic!("Expected 1-8 for rank in FEN record")
          }
        },
        None => panic!("Expected 1-8 for rank in FEN record")
      }
  
      square
    };
  
    let mut parse_en_passant_target_square = || {
      let c = i.borrow_mut().peek().copied();
      match c {
        Some('-') => { i.borrow_mut().next(); },
        Some(_) => { en_passant_target_square = parse_square(); },
        None => panic!("Expected en passant target square in FEN record")
      }
    };
  
    let parse_integer = || {
      let mut integer = String::new();
  
      loop {
        let c = i.borrow_mut().peek().copied();
        match c {
          Some(d) => {
            match d {
              '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                integer.push(d);
                i.borrow_mut().next();
              }
              _ => break
            }
          },
          None => break
        }
      }
  
      assert!(integer.len() > 0, "Expected an integer in FEN record");
      integer.parse::<i32>().unwrap()
    };
  
    let mut parse_halfmove_clock = || {
      halfmove_clock = parse_integer() as u8;
    };
  
    let mut parse_fullmove_counter = || {
      fullmove_counter = parse_integer() as u16;
      assert!(fullmove_counter != 0, "Fullmove counter cannot be zero in FEN record")
    };
  
    parse_piece_placement();
    parse_character(' ');
    parse_side_to_move();
    parse_character(' ');
    parse_castling_ability();
    parse_character(' ');
    parse_en_passant_target_square();
    parse_character(' ');
    parse_halfmove_clock();
    parse_character(' ');
    parse_fullmove_counter();

    Position {
      side_to_move: side_to_move,
      board: Board {
        kings: [white_king, black_king],
        queens: [white_queens, black_queens],
        rooks: [white_rooks, black_rooks],
        bishops: [white_bishops, black_bishops],
        knights: [white_knights, black_knights],
        pawns: [white_pawns, black_pawns],
      },
      castling_ability: CastlingAbility {
        white_kingside: castle_white_kingside,
        white_queenside: castle_white_queenside,
        black_kingside: castle_black_kingside,
        black_queenside: castle_black_queenside,
      },
      halfmove_clock: halfmove_clock,
      fullmove_counter: fullmove_counter,
    }
  }

  fn moves(&self) -> Vec<Move> {
    Vec::new()
  }
}

fn main() {
}

/*
#[allow(non_camel_case_types)]
enum Squares {
  a8 = 56, b8 = 57, c8 = 58, d8 = 59, e8 = 60, f8 = 61, g8 = 62, h8 = 63,
  a7 = 48, b7 = 49, c7 = 50, d7 = 51, e7 = 52, f7 = 53, g7 = 54, h7 = 55,
  a6 = 40, b6 = 41, c6 = 42, d6 = 43, e6 = 44, f6 = 45, g6 = 46, h6 = 47,
  a5 = 32, b5 = 33, c5 = 34, d5 = 35, e5 = 36, f5 = 37, g5 = 38, h5 = 39,
  a4 = 24, b4 = 25, c4 = 26, d4 = 27, e4 = 28, f4 = 29, g4 = 30, h4 = 31,
  a3 = 16, b3 = 17, c3 = 18, d3 = 19, e3 = 20, f3 = 21, g3 = 22, h3 = 23,
  a2 =  8, b2 =  9, c2 = 10, d2 = 11, e2 = 12, f2 = 13, g2 = 14, h2 = 15,
  a1 =  0, b1 =  1, c1 =  2, d1 =  3, e1 =  4, f1 =  5, g1 =  6, h1 =  7,
}
*/


