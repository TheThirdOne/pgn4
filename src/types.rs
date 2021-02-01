use fen4::{Board, Color, Position};

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
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Move {
    Checkmate,
    Timeout,
    Stalemate,
    Resign,
    KingCastle(usize),
    QueenCastle(usize),
    Normal(BasicMove),
}

#[derive(PartialEq, Clone, Debug)]
pub enum GameResult {
    Error,
    Aborted,
    Team(bool, bool),
    FFA([u16; 4]),
}

/// Moves containing to and from positions
///
/// If the piece moved is a pawn, `piece` = 'P'. Similarly, is a pawn is captured, `captured` = Some('P').
/// `checks` should be less than 4 and `mates` should be less than 3 (mating three players simultaneously doesn't get notated).
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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
#[derive(PartialEq, Clone)]
pub struct QuarterTurn {
    pub main: Move,
    pub modifier: Option<Move>,
    pub extra_stalemate: bool,
    pub description: Option<String>,
    pub alternatives: Vec<Vec<Turn>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    // critical options
    pub red_teammate: Color,
    pub initial_board: Board,

    // general options
    pub king_of_the_hill: bool,
    pub antichess: bool,
    pub promote_to: Vec<char>,
    pub dead_wall: bool,
    pub en_passant: bool,
    pub capture_the_king: bool,
    pub pawn_promotion_rank: usize,
    pub ncheck: usize,
    pub chess960: u16,
    // ffa specific options
    pub ffa_dead_king_walking: bool,
    pub ffa_takeover: bool,
    pub ffa_opp_x: u16,
    pub ffa_points_for_mate: u16,
    pub ffa_play_for_mate: bool,
}
