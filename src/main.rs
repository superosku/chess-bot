#[derive(Clone)]
struct Board {
    values: [[u8; 8]; 8]
}

impl Board {
    fn new() -> Board {
        Board {
            values: [
                [0b0010, 0b0011, 0b0100, 0b0101, 0b0110, 0b0100, 0b011, 0b0010],
                [0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b001, 0b0001],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b1001],
                [0b1010, 0b1011, 0b1100, 0b1101, 0b1110, 0b1100, 0b1011, 0b1010],

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
                match piece {
                    0 => { print!(" ") }

                    1 => { print!("p") }
                    2 => { print!("r") }
                    3 => { print!("k") }
                    4 => { print!("b") }
                    5 => { print!("q") }
                    6 => { print!("k") }

                    9 => { print!("P") }
                    10 => { print!("R") }
                    11 => { print!("K") }
                    12 => { print!("B") }
                    13 => { print!("Q") }
                    14 => { print!("K") }

                    _ => { print!("?") }
                }
            }
            println!("");
        }
    }

    fn do_move(&self, fx: usize, fy: usize, tx: usize, ty: usize) -> Board {
        let mut new_board = self.clone();
        new_board.values[ty][tx] = new_board.values[fy][fx];
        new_board.values[fy][fx] = 0;
        new_board
    }

    fn is_in_board(&self, x: i8, y: i8) -> bool {
        x >= 0 && y >= 0 && x < 8 && y < 8
    }

    fn is_free(&self, x: i8, y: i8) -> bool {
        self.values[y as usize][x as usize] == 0
    }

    fn is_enemy(&self, x: i8, y: i8, turn: u8) -> bool {
        self.values[y as usize][x as usize] >> 3 != turn && !self.is_free(x, y)
    }

    fn valid_moves(&self, turn: u8) -> Vec<Board> {
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

        let mut valid_moves = vec![]; // Vec<Board>::new();

        for (y, row) in self.values.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                let colorless_piece = if
                (turn == 0 && (piece & 0b1000 == 0)) ||
                    (turn == 1 && (piece & 0b1000 != 0))
                { piece & 0b111 } else { 0 };
                if colorless_piece != 0 {
                    // Pawns
                    if colorless_piece == 1 {
                        let dir = if turn == 0 { 1 as i8 } else { -1 };
                        if self.is_in_board(x as i8, y as i8 + dir) &&
                            self.is_free(x as i8, y as i8 + dir)
                        {
                            valid_moves.push(
                                self.do_move(x, y, x, (y as i8 + dir) as usize)
                            )
                        }
                        if self.is_in_board(x as i8 + 1, y as i8 + dir) &&
                            self.is_enemy(x as i8 + 1, y as i8 + dir, turn)
                        {
                            valid_moves.push(
                                self.do_move(x, y, x + 1, (y as i8 + dir) as usize)
                            )
                        }
                        if self.is_in_board(x as i8 - 1, y as i8 + dir) &&
                            self.is_enemy(x as i8 - 1, y as i8 + dir, turn)
                        {
                            valid_moves.push(
                                self.do_move(x, y, x - 1, (y as i8 + dir) as usize)
                            )
                        }
                        if self.is_in_board(x as i8, y as i8 + dir * 2) &&
                            self.is_free(x as i8, y as i8 + dir * 2) &&
                            self.is_free(x as i8, y as i8 + dir)
                        {
                            valid_moves.push(
                                self.do_move(x, y, x, (y as i8 + dir * 2) as usize)
                            )
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
                                // print!("{} {} {} {} {} ", x, y, newx, newy,
                                //        self.is_in_board(newx, newy),
                                //        // self.is_enemy(newx, newy, turn),
                                //        // self.is_free(newx, newy),
                                // );
                                if !self.is_in_board(newx, newy) {
                                    // println!("");
                                    break;
                                }
                                let is_enemy = self.is_enemy(newx, newy, turn);
                                let is_free = self.is_free(newx, newy);
                                // println!("{} {}", is_enemy, is_free);
                                if is_free || is_enemy {
                                    valid_moves.push(
                                        self.do_move(x, y, newx as usize, newy as usize)
                                    );
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
                                valid_moves.push(
                                    self.do_move(x, y, newx as usize, newy as usize)
                                )
                            }
                        }
                    }
                }
            }
        }

        valid_moves
    }

    fn req_play(&self, turn: u8, depth: i8) -> i64 {
        if depth == 0 {
            return 1
        }
        let mut counter = 0;
        for new_position in self.valid_moves(turn) {
            counter += new_position.req_play(if turn == 1 {0} else {1}, depth - 1)
        }
        counter
    }
}

fn main() {
    let board = Board::new();
    board.print();
    for depth in 0..10 {
        let step_count = board.req_play(0, depth);
        println!("Depth: {} Steps: {}", depth, step_count);
    }

    // board.req_play(0, 6);

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
