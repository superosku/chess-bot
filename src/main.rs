#[derive(Clone)]
struct Board {
    last_move_x: u8,
    last_move_y: u8,
    values: [[u8; 8]; 8],
}

impl Board {
    fn new() -> Board {
        Board {
            last_move_x: 100,
            last_move_y: 100,
            values: [
                [0b10010, 0b10011, 0b10100, 0b10101, 0b10110, 0b10100, 0b10011, 0b10010],
                [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0b11001, 0b11001, 0b11001, 0b11001, 0b11001, 0b11001, 0b11001, 0b11001],
                [0b11010, 0b11011, 0b11100, 0b11101, 0b11110, 0b11100, 0b11011, 0b11010],

                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0b001, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0b1001, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],

                // // [0b1001,0b1001,0b1001,0b1001,0b1001,0b1001,0b1001,0b1001],
                // // [0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b1001],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // // [0, 0, 0b0010, 0, 0, 0, 0, 0],
                // // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b001, 0b0001],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // // [0, 0, 0b1010, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // // [0b1101, 0, 0, 0, 0, 0, 0, 0],
                // // [0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b001, 0b0001],
                // // [0b0001,0b0001,0b0001,0b0001,0b0001,0b0001,0b001,0b0001],
                // [0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b1001],
                // [0b1010, 0b1011, 0b1100, 0b1101, 0b1110, 0b1100, 0b1011, 0b1010],
            ],
        }
    }

    fn print(&self) {
        println!("********");
        for row in self.values.iter() {
            for piece in row {
                match piece & 0b1111 {
                    0 => { print!(" ") }

                    1 => { print!("p") }
                    2 => { print!("r") }
                    3 => { print!("n") }
                    4 => { print!("b") }
                    5 => { print!("q") }
                    6 => { print!("k") }

                    9 => { print!("P") }
                    10 => { print!("R") }
                    11 => { print!("N") }
                    12 => { print!("B") }
                    13 => { print!("Q") }
                    14 => { print!("K") }

                    _ => { print!("?") }
                }
            }
            println!("");
        }
    }

    fn has_kings(&self) -> bool {
        let mut small_king = false;
        let mut big_king = false;
        for row in self.values.iter() {
            for piece in row {
                match piece & 0b1111 {
                    6 => { small_king = true }
                    14 => { big_king = true }
                    _ => {}
                }
            }
        }
        small_king && big_king
    }

    fn do_move(&self, fx: usize, fy: usize, tx: usize, ty: usize) -> Board {
        let mut new_board = self.clone();
        new_board.values[ty][tx] = new_board.values[fy][fx] & !0b10000; // Remove not moved flag
        new_board.values[fy][fx] = 0;
        new_board.last_move_x = tx as u8;
        new_board.last_move_y = ty as u8;
        new_board
    }

    fn is_in_board(&self, x: i8, y: i8) -> bool {
        x >= 0 && y >= 0 && x < 8 && y < 8
    }

    fn is_free(&self, x: i8, y: i8) -> bool {
        self.values[y as usize][x as usize] == 0
    }

    fn is_enemy(&self, x: i8, y: i8, turn: u8) -> bool {
        (self.values[y as usize][x as usize] >> 3 & 1) != turn && !self.is_free(x, y)
    }

    fn is_en_passantable_pawn(&self, x: i8, y: i8, turn: u8) -> bool {
        self.last_move_y == y as u8 // Is lastly moved piece
            && self.last_move_x == x as u8 // ._.
            && (self.values[y as usize][x as usize] >> 3 & 1) != turn // Is right color
            && self.values[y as usize][x as usize] & 0b111 == 1 // Is pawn
            && self.values[y as usize][x as usize] & 0b100000 == 1 // Did move twice when last moved
    }

    fn prospective_moves(&self, turn: u8) -> Vec<Board> {
        let rook_moves = [
            (-1 as i8, 0 as i8),
            (1, 0),
            (0, 1),
            (0, -1),
        ];
        let bichop_moves = [
            (-1 as i8, -1 as i8),
            (1, 1),
            (-1, 1),
            (1, -1),
        ];
        let queen_moves = [
            (-1 as i8, 0 as i8),
            (1, 0),
            (0, 1),
            (0, -1),
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
        ];

        let mut moves = vec![]; // Vec<Board>::new();

        for (y, row) in self.values.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                let colorless_piece = if
                (turn == 0 && (piece & 0b1000 == 0))
                    || (turn == 1 && (piece & 0b1000 != 0))
                { piece & 0b111 } else { 0 };
                let piece_unmoved = piece & 0b10000 != 0;
                if colorless_piece != 0 {
                    // Pawns
                    if colorless_piece == 1 {
                        let dir = if turn == 0 { 1 as i8 } else { -1 };
                        // Move one
                        if self.is_in_board(x as i8, y as i8 + dir) &&
                            self.is_free(x as i8, y as i8 + dir)
                        {
                            let move_prospect = self.do_move(x, y, x, (y as i8 + dir) as usize);
                            moves.push(move_prospect)
                        }
                        // Eat
                        if self.is_in_board(x as i8 + 1, y as i8 + dir) &&
                            self.is_enemy(x as i8 + 1, y as i8 + dir, turn)
                        {
                            let move_prospect = self.do_move(x, y, x + 1, (y as i8 + dir) as usize);
                            moves.push(move_prospect)
                        }
                        if self.is_in_board(x as i8 - 1, y as i8 + dir) &&
                            self.is_enemy(x as i8 - 1, y as i8 + dir, turn)
                        {
                            let move_prospect = self.do_move(x, y, x - 1, (y as i8 + dir) as usize);
                            moves.push(move_prospect)
                        }
                        // Move two
                        let ynew = y as i8 + dir * 2;
                        if self.is_in_board(x as i8, ynew)
                            && self.is_free(x as i8, ynew)
                            && self.is_free(x as i8, y as i8 + dir)
                            && piece_unmoved
                        {
                            let mut move_prospect = self.do_move(x, y, x, ynew as usize);
                            // Store for en passant possibility that moved twice on last move..
                            let thing = 0b100000 | move_prospect.values[ynew as usize][x as usize];
                            move_prospect.values[ynew as usize][x as usize] = thing;
                            moves.push(move_prospect)
                        }
                        // En passant
                        if self.is_in_board(x as i8 + 1, y as i8 + dir)
                            && self.is_in_board(x as i8, y as i8 + dir)
                            && self.is_en_passantable_pawn(x as i8 + 1, y as i8, turn)
                        {
                            let mut move_prospect = self.do_move(x, y, x + 1, (y as i8 + dir) as usize);
                            move_prospect.values[y][x + 1] = 0;
                            // move_prospect.print();
                            moves.push(move_prospect)
                        }
                        if self.is_in_board(x as i8 - 1, y as i8 + dir)
                            && self.is_in_board(x as i8, y as i8 + dir)
                            && self.is_en_passantable_pawn(x as i8 - 1, y as i8, turn)
                        {
                            let mut move_prospect = self.do_move(x, y, x - 1, (y as i8 + dir) as usize);
                            move_prospect.values[y][x - 1] = 0;
                            // move_prospect.print();
                            moves.push(move_prospect)
                        }
                    }
                    // King
                    if colorless_piece == 6 {
                        for (dx, dy) in queen_moves.iter() {
                            let newx = x as i8 + dx;
                            let newy = y as i8 + dy;

                            if self.is_in_board(newx, newy)
                                && (
                                self.is_free(newx, newy) || self.is_enemy(newx, newy, turn)
                            ) {
                                let move_prospect = self.do_move(x, y, newx as usize, newy as usize);
                                moves.push(move_prospect);
                            }
                        }
                    }
                    // Rook
                    // Bishop
                    // Queen
                    if colorless_piece == 2 ||
                        colorless_piece == 4 ||
                        colorless_piece == 5
                    {
                        for (xdir, ydir) in if colorless_piece == 2 { rook_moves.iter() } else if colorless_piece == 4 { bichop_moves.iter() } else { queen_moves.iter() }
                        {
                            let mut multiplier = 1;
                            while true {
                                let newx = x as i8 + xdir * multiplier;
                                let newy = y as i8 + ydir * multiplier;
                                if !self.is_in_board(newx, newy) {
                                    break;
                                }
                                let is_enemy = self.is_enemy(newx, newy, turn);
                                let is_free = self.is_free(newx, newy);
                                if is_free || is_enemy {
                                    let move_prospect = self.do_move(x, y, newx as usize, newy as usize);
                                    moves.push(move_prospect);
                                    if is_enemy {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                                multiplier += 1;
                            }
                        }
                    }
                    // Horse
                    if colorless_piece == 3 {
                        for (xdiff, ydiff) in [
                            (1 as i8, 2 as i8),
                            (2, 1),
                            (-1, 2),
                            (-2, 1),
                            (1, -2),
                            (2, -1),
                            (-1, -2),
                            (-2, -1),
                        ].iter() {
                            let newx: i8 = x as i8 + xdiff;
                            let newy: i8 = y as i8 + ydiff;
                            if self.is_in_board(newx, newy) &&
                                (
                                    self.is_enemy(newx, newy, turn) ||
                                        self.is_free(newx, newy)
                                )
                            {
                                let move_prospect = self.do_move(x, y, newx as usize, newy as usize);
                                moves.push(move_prospect)
                            }
                        }
                    }
                }
            }
        }

        moves
    }

    // Checks if the king can be eaten by enemy from this position -> not valid position
    fn is_valid(&self, turn: u8) -> bool {
        for position in self.prospective_moves(turn) {
            if !position.has_kings() {
                return false;
            }
        }
        return true;
    }

    // Filter out non valid moves from prospective moves
    fn valid_moves(&self, turn: u8) -> Vec<Board> {
        let mut valid_moves: Vec<Board> = vec![];
        for prospect in self.prospective_moves(turn) {
            // if !prospect.has_kings() {
            // if prospect.is_valid(turn) {
            if prospect.is_valid(if turn == 1 { 0 } else { 1 }) {
                valid_moves.push(prospect)
            }
        }
        valid_moves
    }

    fn req_play(&self, turn: u8, depth: i8) -> i64 {
        // self.print();
        if depth == 0 {
            return 1;
        }
        let mut counter = 0;
        for new_position in self.valid_moves(turn) {
            // println!("DEPTH {}", depth);
            // new_position.print();
            counter += new_position.req_play(if turn == 1 { 0 } else { 1 }, depth - 1)
        }
        counter
    }
}

fn main() {
    let board = Board::new();
    for (depth, correct) in [
        1, 20, 400, 8902, 197281, 4865609, 119060324
    ].iter().enumerate() {
        let step_count = board.req_play(0, depth as i8);
        println!("Depth: {} Correct: {} Steps: {} Diff: {}", depth, correct, step_count, step_count - correct);
    }

    // board.print();
    // for bb in board.valid_moves(0) {
    //     bb.print();
    // }

    // board.req_play(0, 2);

    // println!("**********************");
    // println!("* Moves for 0:");
    // println!("**********************");
    // for new_position in board.valid_moves(0) {
    //     new_position.print();
    // }
    // println!("**********************");
    // println!("* Moves for 1:");
    // println!("**********************");
    // for new_position in board.valid_moves(1) {
    //     new_position.print();
    // }
}
