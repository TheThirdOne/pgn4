use fen4::{Board, Color, Position};

/// Any move e.g. Ka4-b5, T, O-O, ...
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

/// Parsed version of the PGN4 tag "Result"
#[derive(PartialEq, Clone, Debug)]
pub enum GameResult {
    Error,
    Aborted,
    /// (RY win, BG win); (true, true) is invalid and (false,false) is a draw
    Team(bool, bool),
    /// Points for Red - Green in order
    FFA([u16; 4]),
}

/// The most common type of move that contains to and from positions
///
/// The mapping from Struct to String is mostly straightforward. The struct elements are in the order they are serialized in.
/// If the piece moved is a pawn, `piece` = 'P'. Similarly, is a pawn is captured, `captured` = Some('P').
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

/// Representation of pgn4 file
///
/// TODO: general description
///
/// Notably, Chess.com does not support variations within variations whereas this
/// tool does. If you try to import a pgn4 with recursive variations into Chess.com,
/// you will get errors.
#[derive(PartialEq, Clone, Debug)]
pub struct PGN4 {
    /// PGN4 files have a list of key value pairs in the beginning that specify variants, time control, and other metadata
    pub bracketed: Vec<(String, String)>,
    /// The game is stored as a list of [Turns](`Turn`) which hold up to 4
    /// [QuarterTurns]('QuarterTurn') each. Unfortunately, this cannot be
    /// simplified to a single list without compromising round-trip serilization
    /// or including enough logic to deduce which player's turn it is.
    ///
    /// If you need to follow sub-variations, please use a [`Visitor`](`crate::VisitorCommon`)
    pub turns: Vec<Turn>,
}

/// Representation of a single Turn
#[derive(PartialEq, Clone, Debug)]
pub struct Turn {
    /// The number that is shown at the beginning of a turn. 0 if no number should be shown.
    pub number: usize,
    /// If there should be two '.'s at the beginning of the turn (after the number). This is used when working in sub-variations.
    pub double_dot: bool,
    /// There are a maximum of 4 quarterturns in a turn, but in FFA there are many times when there are less than 4.
    pub turns: Vec<QuarterTurn>,
}

/// A single move optionally with a description and alternatives
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
#[derive(PartialEq, Clone)]
pub struct QuarterTurn {
    pub main: Move,
    /// If FFA with zombies, it is possible to resign / timeout and still move.
    pub modifier: Option<Move>,
    /// In Antichess it is possible to stalemate another player by capturing. This represents that mate-like 'S'
    pub extra_stalemate: bool,
    /// A description for the move notated like " { description goes here } "
    pub description: Option<String>,
    /// Possible alternative moves that could be played. They are notated using parenthesis enclosing a full set of turns.
    /// Multiple different variations can be notated like "( subvariation1 ) ( subvariation2 ) ".
    pub alternatives: Vec<Vec<Turn>>,
}

/// Representation of different variants of 4 player chess
///
/// This has been made to closely resemble the setup for starting games on Chess.com.
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
