
#[derive(Clone)]
enum PieceColor {
    White,
    Black
}

#[derive(Clone)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(Clone)]
struct Piece {
    color: PieceColor,
    name: PieceType
}

impl Piece {
    fn new(color: PieceColor, name: PieceType) -> Option<Piece> {
        Some(Piece { color, name })
    }
}

type Point = (usize, char);

const BOARD_SIZE: usize = 8;
const PIECES: [char; 12] = ['♔', '♕', '♖', '♗', '♘', '♙', '♚', '♛', '♜', '♝', '♞', '♟'];

fn main() {
    let mut board: Vec<Vec<Option<Piece>>> = Vec::new();
    for _ in 0..BOARD_SIZE {
        let mut row: Vec<Option<Piece>> = Vec::new();
        for _ in 0..BOARD_SIZE {
            row.push(None);
        }
        board.push(row);
    }

    // pawns
    for i in 0..BOARD_SIZE {
        board[1][i] = Piece::new(PieceColor::White, PieceType::Pawn);
    }
    for i in 0..BOARD_SIZE {
        board[6][i] = Piece::new(PieceColor::Black, PieceType::Pawn);
    }

    // setup white
    board[0][0] = Piece::new(PieceColor::White, PieceType::Rook);
    board[0][1] = Piece::new(PieceColor::White, PieceType::Knight);
    board[0][2] = Piece::new(PieceColor::White, PieceType::Bishop);
    board[0][3] = Piece::new(PieceColor::White, PieceType::King);
    board[0][4] = Piece::new(PieceColor::White, PieceType::Queen);
    board[0][5] = Piece::new(PieceColor::White, PieceType::Bishop);
    board[0][6] = Piece::new(PieceColor::White, PieceType::Knight);
    board[0][7] = Piece::new(PieceColor::White, PieceType::Rook);

    // setup black
    board[7][0] = Piece::new(PieceColor::Black, PieceType::Rook);
    board[7][1] = Piece::new(PieceColor::Black, PieceType::Knight);
    board[7][2] = Piece::new(PieceColor::Black, PieceType::Bishop);
    board[7][3] = Piece::new(PieceColor::Black, PieceType::Queen);
    board[7][4] = Piece::new(PieceColor::Black, PieceType::King);
    board[7][5] = Piece::new(PieceColor::Black, PieceType::Bishop);
    board[7][6] = Piece::new(PieceColor::Black, PieceType::Knight);
    board[7][7] = Piece::new(PieceColor::Black, PieceType::Rook);

    move_piece(&mut board, (7, 'b'), (3, 'b'));

    draw_board(&board);
}

fn draw_board(board: &Vec<Vec<Option<Piece>>>) {
    print!("  ");
    for i in 0..BOARD_SIZE {
        print!("{i} ");
    }
    println!();

    let mut y = 'a';
    for row in board {
        print!("{y} ");
        y = ((y as u8) + 1) as char;
        for col in row {
            if let Some(piece) = col {
                print!("{}", get_piece(&piece.color, &piece.name));
            } else {
                print!("■");
            }
            print!(" ");
        }
        println!();
    }
}

fn move_piece(board: &mut Vec<Vec<Option<Piece>>>, (x1, y1): Point, (x2, y2): Point) {
    let y1 = (y1 as usize) - ('a' as usize);
    let y2 = (y2 as usize) - ('a' as usize);

    let piece = board[x1][y1].clone();
    board[x2][y2] = piece;
    board[x1][y1] = None;
}


fn get_piece(color: &PieceColor, name: &PieceType) -> char {
    match color {
        PieceColor::Black => match name {
            PieceType::King => PIECES[0],
            PieceType::Queen => PIECES[1],
            PieceType::Rook => PIECES[2],
            PieceType::Bishop => PIECES[3],
            PieceType::Knight => PIECES[4],
            PieceType::Pawn => PIECES[5]
        }
        PieceColor::White => match name  {
            PieceType::King => PIECES[6],
            PieceType::Queen => PIECES[7],
            PieceType::Rook => PIECES[8],
            PieceType::Bishop => PIECES[9],
            PieceType::Knight => PIECES[10],
            PieceType::Pawn => PIECES[11]
        }
    }
}