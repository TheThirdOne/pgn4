#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BasicMove {
    pub piece: char,
    pub from: Position,
    pub captured: Option<char>,
    pub to: Position,
    pub checks: usize,
    pub mates: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Move {
    Checkmate,
    Timeout,
    Stalemate,
    Resign,
    Normal(BasicMove),
}
/*
pub enum BracketType {
    GameNr,
    TimeControl,
    StartFen4,
    Variant,
    RuleVariants,
    Red,
    RedElo,
    Blue,
    BlueElo,
    Yellow,
    YellowElo,
    Green,
    GreenElo,
    Result,
    Termination,
    Site,
    Date,
    CurrentMove,
}*/

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

#[derive(PartialEq, Clone, Debug)]
pub struct QuarterTurn {
    pub main: Move,
    pub description: Option<String>,
    pub alternatives: Vec<Vec<Turn>>,
}
