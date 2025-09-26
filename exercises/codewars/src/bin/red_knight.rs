/* Chess knight pursuit pawns.

Red Knight is chasing two pawns. Which pawn will be caught, and where?

Input / Output
Input will be two integers:

N / n (Ruby) vertical position of Red Knight (0 or 1).
P / p (Ruby) horizontal position of two pawns (between 2 and 1000000).
Output has to be a tuple (python, haskell, Rust, prolog, C#), an array (javascript, ruby), an object (java), or a structure (C) with:

"Black" or "White" - which pawn was caught
Where it was caught (horizontal position)

* Red Knight will always start at horizontal position 0.
* The black pawn will always be at the bottom (vertical position 1).
* The white pawn will always be at the top (vertical position 0).
* The pawns move first, and they move simultaneously.
* Red Knight moves 2 squares forward and 1 up or down.
* Pawns always move 1 square forward.
* Both pawns will start at the same horizontal position.
*/
#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn update_position(&mut self, x: u32, y: u32) {
        self.x += x;
        self.y += y;
        // For the knight
        if self.y > 1 {
            self.y = 0;
        }
    }

    fn is_same(&self, position: &Position) -> bool {
        (self.x == position.x) & (self.y == position.y)
    }
}

fn caught_piece(knight: u32, pawns: u32) -> (String, u32) {
    // Start game
    let mut red_knight = Position{x: knight, y: 0};
    let mut white_pawn = Position{x:pawns, y:0};
    let mut black_pawn = Position{x:pawns, y:1};
    // To prevent infinite loops
    for _iter in 0..100 {
        // Move pawns
        white_pawn.update_position(1, 0);
        black_pawn.update_position(1, 0);
        dbg!(&white_pawn);
        // Move Knight
        red_knight.update_position(2, 1);
        dbg!(&red_knight);
        // Check positions
        if red_knight.is_same(&white_pawn) {
            return ("White".to_string(), red_knight.x);
        }
        if red_knight.is_same(&black_pawn) {
            return ("Black".to_string(), red_knight.x);
        }
    }
    // If does not resolve
    ("Can not calculate after 100 cicles".to_string(), 0)

}

fn main() {
    let (piece, position) = caught_piece(1, 6);
    println!("{piece}, {position}");
}