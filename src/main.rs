use std::collections::HashMap;
use std::env;


#[derive(Clone)]
struct Board {
    last_move_x: u8,
    last_move_y: u8,
    values: [[u8; 8]; 8],
}

fn number_to_letter(number: i8) -> String {
    String::from(match number {
        0 => { "a" }
        1 => { "b" }
        2 => { "c" }
        3 => { "d" }
        4 => { "e" }
        5 => { "f" }
        6 => { "g" }
        7 => { "h" }
        _ => { "?" }
    })
}

fn parse_info(info: (i8, i8, i8, i8)) -> String {
    let (x, y, tx, ty) = info;
    let xl = number_to_letter(x);
    let txl = number_to_letter(tx);
    return format!("{}{}{}{}", xl, 9 - (y + 1), txl, 9 - (ty + 1));
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

                // [0b10010, 0, 0, 0, 0b10110, 0, 0, 0b10010],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0, 0, 0, 0, 0, 0, 0, 0],
                // [0b11010, 0, 0, 0, 0b11110, 0b11010, 0, 0],

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

    fn from_edwards_notation(input: &String) -> (u8, Board) {
        // println!("asdf {}", input);

        let input_parts: Vec<&str> = input.split(" ").collect();
        let board_content_part = input_parts[0].clone();

        // let turn = 0;
        //
        // println!("JEJE {}", input_parts[0]);
        // println!("JEJE {}", input_parts[1]);
        // println!("JEJE {}", input_parts[2]);

        let turn = if input_parts[1] == "b" { 0 } else { 1 };

        let mut board = Board {
            last_move_x: 100,
            last_move_y: 100,
            values: [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
        };

        let mut y = 0;
        let mut x = 0;

        for char in board_content_part.chars() {
            match char {
                '/' => {
                    y += 1;
                    x = 0;
                }
                '1' => { x += 1; }
                '2' => { x += 2; }
                '3' => { x += 3; }
                '4' => { x += 4; }
                '5' => { x += 5; }
                '6' => { x += 6; }
                '7' => { x += 7; }
                '8' => { x += 8; }
                'p' => {
                    board.values[y][x] = if y == 1 { 0b10001 } else { 0b00001 };
                    x += 1;
                }
                'r' => {
                    board.values[y][x] = 0b10010;
                    x += 1;
                }
                'n' => {
                    board.values[y][x] = 0b0011;
                    x += 1;
                }
                'b' => {
                    board.values[y][x] = 0b0100;
                    x += 1;
                }
                'q' => {
                    board.values[y][x] = 0b0101;
                    x += 1;
                }
                'k' => {
                    board.values[y][x] = 0b10110;
                    x += 1;
                }
                'P' => {
                    board.values[y][x] = if y == 6 { 0b11001 } else { 0b01001 };
                    x += 1;
                }
                'R' => {
                    board.values[y][x] = 0b11010;
                    x += 1;
                }
                'N' => {
                    board.values[y][x] = 0b1011;
                    x += 1;
                }
                'B' => {
                    board.values[y][x] = 0b1100;
                    x += 1;
                }
                'Q' => {
                    board.values[y][x] = 0b1101;
                    x += 1;
                }
                'K' => {
                    board.values[y][x] = 0b11110;
                    x += 1;
                }
                _ => { println!("BOO") }
            }
            // println!("Now at {} {}", x, y);
        }

        (turn, board)
    }

    fn hash(&self) -> u64 {
        let mut val: u64 = 0;

        val = self.last_move_x as u64 + (val << 6) + (val << 16) - val;
        val = self.last_move_y as u64 + (val << 6) + (val << 16) - val;

        for row in self.values.iter() {
            for piece in row {
                val = *piece as u64 + (val << 6) + (val << 16) - val;
            }
        }

        val
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
        new_board.values[ty][tx] = new_board.values[fy][fx] & !0b110000; // Remove not moved flag and last move was pawn twice flags
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

    fn holds_piece_of(&self, x: i8, y: i8, turn: u8) -> bool {
        if self.is_free(x, y) {
            return false;
        }
        (self.values[y as usize][x as usize] >> 3 & 1) == turn
    }

    fn is_enemy(&self, x: i8, y: i8, turn: u8) -> bool {
        (self.values[y as usize][x as usize] >> 3 & 1) != turn && !self.is_free(x, y)
    }

    fn is_en_passantable_pawn(&self, x: i8, y: i8, turn: u8) -> bool {
        let ret_val = self.last_move_y == y as u8 // Is lastly moved piece
            && self.last_move_x == x as u8 // ._.
            && (self.values[y as usize][x as usize] >> 3 & 1) != turn // Is right color
            && self.values[y as usize][x as usize] & 0b111 == 1 // Is pawn
            && self.values[y as usize][x as usize] & 0b100000 != 0; // Did move twice when last moved

        ret_val
        // false
    }

    fn handle_pawn_promotion(&self, moves: &mut Vec<((i8, i8, i8, i8), Board)>, x: usize, y: usize, tx: usize, newy: i8, turn: u8) {
        let move_prospect = self.do_move(x, y, tx, newy as usize);
        if newy == 0 || newy == 7 {
            for promotion_piece in [2, 3, 4, 5].iter() {
                let right_color_piece = promotion_piece | if turn == 0 { 0b1000 } else { 0b0 };
                let mut promotion_move_prospect = move_prospect.clone();
                promotion_move_prospect.values[newy as usize][tx] = right_color_piece;
                moves.push(((x as i8, y as i8, tx as i8, newy), promotion_move_prospect));
            }
        } else {
            moves.push(((x as i8, y as i8, tx as i8, newy), move_prospect));
        }
    }

    fn is_not_under_attack(&self, x: i8, y: i8, turn: u8) -> bool {
        let enemy_turn = if turn == 0 { 1 } else { 0 };
        for (_, enemy_move) in self.prospective_moves(enemy_turn, true) {
            // for ((x2, y2, tx, ty), enemy_move) in self.prospective_moves(enemy_turn, true) {
            if enemy_move.holds_piece_of(x, y, enemy_turn) {
                return false;
            }
        }
        true
    }

    fn prospective_moves(&self, turn: u8, ignore_some: bool) -> Vec<((i8, i8, i8, i8), Board)> {
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
                        let newy = y as i8 + dir;
                        if !ignore_some { // Can't eat forward
                            if self.is_in_board(x as i8, newy) &&
                                self.is_free(x as i8, newy)
                            {
                                self.handle_pawn_promotion(&mut moves, x, y, x, newy, turn);
                            }
                        }
                        // Eat
                        if self.is_in_board(x as i8 + 1, newy) &&
                            (
                                self.is_enemy(x as i8 + 1, newy, turn)
                                    || ignore_some // Needed for castling restriciton checks
                            )
                        {
                            self.handle_pawn_promotion(&mut moves, x, y, x + 1, newy, turn);
                        }
                        // if x == 6
                        //     && y == 6
                        //     && self.is_in_board(x as i8 - 1, newy)
                        // {
                        //     println!("HMHMHM {} {}", self.is_enemy(x as i8 - 1, newy, turn), ignore_some)
                        // }
                        if self.is_in_board(x as i8 - 1, newy) &&
                            (
                                self.is_enemy(x as i8 - 1, newy, turn)
                                    || ignore_some // Needed for castling restriciton checks
                            )
                        {
                            self.handle_pawn_promotion(&mut moves, x, y, x - 1, newy, turn);
                            // println!(
                            //     "HEHEHEHE {} {}",
                            //     moves[moves.len() - 1].1.values[newy as usize][x-1],
                            //     moves[moves.len() - 1].1.values[7][5],
                            // );
                        }
                        // Move two
                        let ynew = y as i8 + dir * 2;
                        if self.is_in_board(x as i8, ynew)
                            && self.is_free(x as i8, ynew)
                            && self.is_free(x as i8, y as i8 + dir)
                            && piece_unmoved
                        {
                            let mut move_prospect = self.do_move(x, y, x, ynew as usize);
                            //Store for e n passant possibility that moved twice on last move..
                            let thing = 0b100000 | move_prospect.values[ynew as usize][x as usize];
                            move_prospect.values[ynew as usize][x as usize] = thing;
                            moves.push(((x as i8, y as i8, x as i8, ynew), move_prospect))
                        }
                        // En passant
                        if self.is_in_board(x as i8 + 1, y as i8 + dir)
                            && self.is_in_board(x as i8, y as i8 + dir)
                            && self.is_en_passantable_pawn(x as i8 + 1, y as i8, turn)
                        {
                            let mut move_prospect = self.do_move(x, y, x + 1, (y as i8 + dir) as usize);
                            move_prospect.values[y][x + 1] = 0;
                            moves.push(((x as i8, y as i8, (x + 1) as i8, ynew), move_prospect))
                        }
                        if self.is_in_board(x as i8 - 1, y as i8 + dir)
                            && self.is_in_board(x as i8, y as i8 + dir)
                            && self.is_en_passantable_pawn(x as i8 - 1, y as i8, turn)
                        {
                            let mut move_prospect = self.do_move(x, y, x - 1, (y as i8 + dir) as usize);
                            move_prospect.values[y][x - 1] = 0;
                            // move_prospect.print();
                            moves.push(((x as i8, y as i8, (x - 1) as i8, ynew), move_prospect))
                        }
                    }
                    // King
                    if colorless_piece == 6 {
                        // Moving
                        for (dx, dy) in queen_moves.iter() {
                            let newx = x as i8 + dx;
                            let newy = y as i8 + dy;
                            if self.is_in_board(newx, newy)
                                && (
                                self.is_free(newx, newy) || self.is_enemy(newx, newy, turn)
                            ) {
                                let move_prospect = self.do_move(x, y, newx as usize, newy as usize);
                                moves.push(((x as i8, y as i8, newx as i8, newy as i8), move_prospect))
                            }
                        }
                        // Castling queen side
                        if !ignore_some {
                            if x == 4
                                && (y == 0 || y == 7)
                                && self.values[y][x] & 0b10000 != 0
                                && self.values[y][0] & 0b10000 != 0
                                && self.is_free(1, y as i8)
                                && self.is_free(2, y as i8)
                                && self.is_free(3, y as i8)
                                && self.is_not_under_attack(4, y as i8, turn)
                                && self.is_not_under_attack(3, y as i8, turn)
                                && self.is_not_under_attack(2, y as i8, turn)
                            {
                                let move_prospect = self
                                    .do_move(4, y, 2, y)
                                    .do_move(0, y, 3, y);
                                moves.push(((4 as i8, y as i8, 2 as i8, y as i8), move_prospect))
                            }
                            // Castling king side
                            if x == 4
                                && (y == 0 || y == 7)
                                && self.values[y][x] & 0b10000 != 0
                                && self.values[y][7] & 0b10000 != 0
                                && self.is_free(6, y as i8)
                                && self.is_free(5, y as i8)
                                && self.is_not_under_attack(4, y as i8, turn)
                                && self.is_not_under_attack(5, y as i8, turn)
                                && self.is_not_under_attack(6, y as i8, turn)
                            {
                                // println!("CASTLING 2: {} {} {} {} {}",
                                //          x,
                                //          y,
                                //          self.is_not_under_attack(4, y as i8, turn),
                                //          self.is_not_under_attack(5, y as i8, turn),
                                //          self.is_not_under_attack(6, y as i8, turn)
                                // );
                                let move_prospect = self
                                    .do_move(4, y, 6, y)
                                    .do_move(7, y, 5, y);
                                moves.push(((4 as i8, y as i8, 6 as i8, y as i8), move_prospect))
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
                            loop {
                                let newx = x as i8 + xdir * multiplier;
                                let newy = y as i8 + ydir * multiplier;
                                if !self.is_in_board(newx, newy) {
                                    break;
                                }
                                let is_enemy = self.is_enemy(newx, newy, turn);
                                let is_free = self.is_free(newx, newy);
                                if is_free || is_enemy {
                                    let move_prospect = self.do_move(x, y, newx as usize, newy as usize);
                                    moves.push(((x as i8, y as i8, newx as i8, newy as i8), move_prospect));
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
                                moves.push(((x as i8, y as i8, newx as i8, newy as i8), move_prospect))
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
        for (_, position) in self.prospective_moves(turn, true) { // TODO: Kai voi ignoraa täs?
            if !position.has_kings() {
                return false;
            }
        }
        return true;
    }

    // Filter out non valid moves from prospective moves
    fn valid_moves(&self, turn: u8) -> Vec<((i8, i8, i8, i8), Board)> {
        let mut valid_moves: Vec<((i8, i8, i8, i8), Board)> = vec![];
        for (move_info, prospect) in self.prospective_moves(turn, false) {
            if prospect.is_valid(if turn == 1 { 0 } else { 1 }) {
                valid_moves.push((move_info, prospect))
            }
        }
        // if valid_moves.len() == 0 {
        //     println!("No valid moves")
        // }
        valid_moves
    }

    fn req_play(
        &self,
        turn: u8,
        depth: u8,
        hasher: &mut HashMap<(u64, u8, u8), i64>,
        print_perf_at: u8,
    ) -> i64 {
        if depth == 0 {
            return 1;
        }
        let mut counter = 0;
        for (info, new_position) in self.valid_moves(turn) {
            match hasher.get(
                &(new_position.hash(), turn, depth)
            ) {
                Some(x) => {
                    counter += x;
                }
                _ => {
                    let value = new_position.req_play(
                        if turn == 1 { 0 } else { 1 },
                        depth - 1,
                        hasher,
                        print_perf_at,
                    );
                    if depth == print_perf_at {
                        println!("{} {}", parse_info(info), value);
                    }
                    hasher.insert(
                        (new_position.hash(), turn, depth),
                        value,
                    );
                    counter += value
                }
            }
        }
        counter
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let mut hasher: HashMap<(u64, u8, u8), i64> = HashMap::new();

        let depth = args[1].parse::<u8>().unwrap();
        let edwards_string = args[2].clone();

        let (turn, board) = Board::from_edwards_notation(&edwards_string);

        let total_moves = board.req_play(turn, depth, &mut hasher, depth);

        println!("");
        println!("{}", total_moves);
    } else {
        for (en, moves) in [
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                Vec::from([1, 20, 400, 8902, 197281])
            ),
            (
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
                Vec::from([1, 48, 2039, 97862])
            ),
            (
                "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -",
                Vec::from([1, 14, 191, 2812, 43238])
            ),
            (
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
                Vec::from([1, 6, 264, 9467])
            ),
            (
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
                Vec::from([1, 6, 264, 9467])
            ),
            (
                "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
                Vec::from([1, 44, 1486, 62379])
            ),
            (
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
                Vec::from([1, 46, 2079, 89890])
            ),
        ].iter() {
            let (turn, board) = Board::from_edwards_notation(&String::from(*en));
            for (depth, correct) in moves.iter().enumerate() {
                let mut hasher: HashMap<(u64, u8, u8), i64> = HashMap::new();
                let step_count = board.req_play(turn, depth as u8, &mut hasher, 100);
                println!("Depth: {} Correct: {} Steps: {} Diff: {}", depth, correct, step_count, step_count - correct);
            }
        }
    }

    if false {
        let b = Board::new();
        b.print();
    }
}
