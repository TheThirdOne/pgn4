/// Position on the board e.g. a4
///
/// Both row and col should be in the range 0-13.
#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

/// Any move e.g. Ka4-b5, T, O-O, ...
///
/// There are four classes of move:
///   1. A player leaving the game
///   2. An automatic move following a class 1 move
///   3. Castling
///   4. Standard moves.
///
/// The second class of moves is the result of the variant "DeadKingWalking" in FFA.
/// When you resign or timeout, your remaining pieces die and your king makes moves
/// on its own. This can be notated like "Ka4-b5R"; the R on the end shows that this
/// is not just a normal king move. Alternatively it is possible to be in checkmate
/// following your pieces dying. That is notated like "R#".
///
/// Castling has a strange property that checks are not notated.
#[derive(PartialEq, Clone, Debug)]
pub enum Move {
    Checkmate,
    Timeout,
    TimeoutMate,
    Stalemate,
    Resign,
    ResignMate,
    KingCastle(usize),
    QueenCastle(usize),
    ResignMove(BasicMove),
    TimeoutMove(BasicMove),
    Normal(BasicMove),
}

/// Moves containing to and from positions
///
/// If the piece moved is a pawn, `piece` = 'P'. Similarly, is a pawn is captured, `captured` = Some('P').
/// `checks` should be less than 4 and `mates` should be less than 3 (mating three players simultaneously doesn't get notated).
#[derive(PartialEq, Clone, Debug)]
pub struct BasicMove {
    pub piece: char,
    pub from: Position,
    pub captured: Option<char>,
    pub to: Position,
    pub promotion: Option<char>,
    pub checks: usize,
    pub mates: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PGN4 {
    pub bracketed: Vec<(String, String)>,
    pub turns: Vec<Turn>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Turn {
    pub number: usize,
    pub double_dot: bool,
    pub turns: Vec<QuarterTurn>,
}

/// A single move optionally with a description and alternatives
#[derive(PartialEq, Clone, Debug)]
pub struct QuarterTurn {
    pub main: Move,
    pub description: Option<String>,
    pub alternatives: Vec<Vec<Turn>>,
}
