use fen4::{Position, PositionParseError};
use std::str::FromStr;

use crate::types::*;

use thiserror::Error;

#[derive(Error, PartialEq, Clone, Debug)]
pub enum MoveError {
    #[error("Basic move is malformed.")]
    Other,
    #[error("A move starts with O-O, but is not a correct type of move.")]
    Castle,
    #[error("Unable to parse basic move because {0}")]
    PositionInvalid(#[from] PositionParseError),
}
impl FromStr for BasicMove {
    type Err = MoveError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut iter = string.chars();
        let start = iter.next().ok_or(MoveError::Other)?;
        let (piece, pieceless) = if start.is_ascii_lowercase() {
            ('P', string)
        } else {
            (start, iter.as_str())
        };
        let mateless = pieceless.trim_end_matches('#');
        let checkless = mateless.trim_end_matches('+');

        let mates = pieceless.len() - mateless.len();
        let checks = mateless.len() - checkless.len();

        let (two_pos, promotion) = if let Some(equals) = checkless.find('=') {
            let (left_over, promote) = checkless.split_at(equals);
            let mut iter = promote.chars();
            if iter.next() != Some('=') {
                return Err(MoveError::Other);
            }
            let p = iter.next().ok_or(MoveError::Other)?;
            if iter.next().is_some() {
                return Err(MoveError::Other);
            }
            (left_over, Some(p))
        } else {
            (checkless, None)
        };

        let loc = if let Some(dash) = two_pos.find('-') {
            dash
        } else if let Some(x) = two_pos.find('x') {
            x
        } else {
            return Err(MoveError::Other);
        };
        let (left, tmp) = two_pos.split_at(loc);
        let (mid, mut right) = tmp.split_at(1); // x and - are both ascii and therefore 1 byte
        let from = left.parse::<Position>()?;
        let captured = if mid == "x" {
            let mut iter = right.chars();
            let start = iter.next().ok_or(MoveError::Other)?;
            Some(if start.is_ascii_lowercase() {
                'P'
            } else {
                right = iter.as_str();
                start
            })
        } else {
            None
        };
        let to = right.parse::<Position>()?;
        Ok(BasicMove {
            piece,
            from,
            captured,
            to,
            promotion,
            checks,
            mates,
        })
    }
}

impl FromStr for Move {
    type Err = MoveError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use Move::*;
        Ok(match string {
            "C" => Claim,
            "#" => Checkmate,
            "S" => Stalemate,
            "T" => Timeout,
            "R" => Resign,
            s if s.starts_with("O-O") => {
                let mateless = s.trim_end_matches('#');
                let mates = s.len() - mateless.len();
                match mateless {
                    "O-O-O" => QueenCastle(mates),
                    "O-O" => KingCastle(mates),
                    _ => return Err(MoveError::Castle),
                }
            }
            _ => Normal(string.parse::<BasicMove>()?),
        })
    }
}

struct MovePair {
    main: Move,
    modifier: Option<Move>,
}

impl FromStr for MovePair {
    type Err = MoveError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let break_index = if string.len() == 2 {
            1 // No move is 2 bytes long
        } else if string.len() > 2 {
            if (string.ends_with('R') && !string.ends_with("=R"))
                || (string.ends_with('S') && !string.ends_with("=S"))
                || (string.ends_with('T') && !string.ends_with("=T"))
            {
                string.len() - 1
            } else {
                0
            }
        } else {
            0
        };
        Ok(if break_index == 0 {
            Self {
                main: string.parse()?,
                modifier: None,
            }
        } else {
            Self {
                main: string.get(..break_index).ok_or(MoveError::Other)?.parse()?,
                modifier: Some(string.get(break_index..).ok_or(MoveError::Other)?.parse()?),
            }
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
enum IntermediateError {
    Other(usize),
    MoveErr(MoveError, String, usize),
    Description(usize),
}

fn parse_quarter(string: &str) -> Result<(QuarterTurn, &str), IntermediateError> {
    /// Generally the move is bounded by whitespace, but supporting pgns that don't
    /// have all the neccessary whitespace is good. Notably, whitespace before a new
    ///  line number is critical.
    fn next_move(c: char) -> bool {
        c.is_whitespace()
            || match c {
                '.' | '{' | '(' | ')' => true,
                _ => false,
            }
    }
    use IntermediateError::*;
    let trimmed = string.trim_start();
    if trimmed == "" {
        return Err(Other(trimmed.len()));
    }
    let split = trimmed.find(next_move).unwrap_or(string.len() - 1);
    let (main_str, mut rest) = trimmed.split_at(split);
    let move_pair = main_str
        .trim()
        .parse::<MovePair>()
        .map_err(|m| MoveErr(m, main_str.to_owned(), rest.len()))?;
    let mut description = None;
    let mut alternatives = Vec::new();
    rest = rest.trim_start();

    if let Some(c) = rest.chars().next() {
        if c == '{' {
            let desc_end = rest.find('}').ok_or(Description(rest.len()))?;
            let (mut desc_str, rest_tmp) = rest.split_at(desc_end + 1);
            desc_str = desc_str.strip_prefix("{ ").ok_or(Description(rest.len()))?;
            desc_str = desc_str.strip_suffix(" }").ok_or(Description(rest.len()))?;
            description = Some(desc_str.to_owned());
            rest = rest_tmp;
        }
    } else {
        return Ok((
            QuarterTurn {
                main: move_pair.main,
                modifier: move_pair.modifier,
                description,
                alternatives,
            },
            rest,
        ));
    };

    while let Some(rest_tmp) = rest.strip_prefix('(') {
        rest = rest_tmp;
        let mut turns = Vec::new();
        while rest.chars().next() != Some(')') {
            let (turn, rest_tmp) = parse_turn(rest)?;
            rest = rest_tmp;
            turns.push(turn);
        }
        rest = rest.strip_prefix(')').unwrap().trim_start();
        alternatives.push(turns);
    }
    Ok((
        QuarterTurn {
            main: move_pair.main,
            modifier: move_pair.modifier,
            description,
            alternatives,
        },
        rest,
    ))
}

fn parse_turn(string: &str) -> Result<(Turn, &str), IntermediateError> {
    use IntermediateError::*;
    let trimmed = string.trim_start();
    let dot_loc = trimmed.find('.').ok_or(Other(trimmed.len()))?;
    let (number_str, dots) = trimmed.split_at(dot_loc);
    let number = if number_str == "" {
        0
    } else {
        number_str.parse().map_err(|_| Other(trimmed.len()))?
    };
    let dot = dots.strip_prefix('.').unwrap();
    let (mut rest, double_dot) = if let Some(dotted) = dot.strip_prefix('.') {
        (dotted, true)
    } else {
        (dot, false)
    };
    let mut turns = Vec::new();
    let for_error = rest.len();
    let (qturn, rest_tmp) = parse_quarter(rest)?;
    rest = rest_tmp.trim_start();
    turns.push(qturn);
    while let Some(rest_tmp) = rest.strip_prefix("..") {
        if turns.len() >= 4 {
            return Err(Other(for_error));
        }
        let (qturn, rest_tmp) = parse_quarter(rest_tmp)?;
        rest = rest_tmp.trim_start();
        turns.push(qturn);
    }
    Ok((
        Turn {
            number,
            double_dot,
            turns,
        },
        rest,
    ))
}

#[derive(Error, PartialEq, Clone, Debug)]
pub enum PGN4Error {
    #[error("Some error occured at {0}")]
    Other(ErrorLocation),
    #[error("Tag starting at {0} is malformed")]
    BadTagged(ErrorLocation),
    #[error("Move \"{1}\" at {2} failed to parse. {0}")]
    BadMove(MoveError, String, ErrorLocation),
    #[error("Description starting at {0} is malformed")]
    BadDescription(ErrorLocation),
}

#[derive(PartialEq, Clone, Debug)]
pub struct ErrorLocation {
    pub line: usize,
    pub column: usize,
    pub raw_offset: usize,
}

impl std::fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {} column {}", self.line, self.column)
    }
}

impl FromStr for PGN4 {
    type Err = PGN4Error;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut bracketed = Vec::new();
        let mut rest = string;
        while let Some(rest_tmp) = rest.strip_prefix('[') {
            let label_end = rest_tmp.find(|c: char| c.is_whitespace()).unwrap_or(0);
            let (label, middle) = rest_tmp.split_at(label_end);
            rest = middle
                .trim_start()
                .strip_prefix('"')
                .ok_or_else(|| make_tagged(rest_tmp, string))?;

            let value_end = rest
                .find('"')
                .ok_or_else(|| make_tagged(rest_tmp, string))?;
            let (value, end) = rest.split_at(value_end);
            rest = end
                .strip_prefix("\"]")
                .ok_or_else(|| make_tagged(rest_tmp, string))?
                .trim_start();

            bracketed.push((label.to_owned(), value.to_owned()));
        }
        let mut turns = Vec::new();
        while rest != "" {
            let (turn, rest_tmp) = parse_turn(rest).map_err(|ie| add_details(ie, string))?;
            rest = rest_tmp;
            turns.push(turn);
        }
        Ok(PGN4 { bracketed, turns })
    }
}

fn map_location(bytes_left: usize, base: &str) -> ErrorLocation {
    let front = base.split_at(base.len() - bytes_left).0;
    let from_last_newline = front.lines().last().unwrap();
    let line = front.lines().count();
    ErrorLocation {
        line,
        column: from_last_newline.chars().count(),
        raw_offset: front.len(),
    }
}

fn make_tagged(rest: &str, string: &str) -> PGN4Error {
    PGN4Error::BadTagged(map_location(rest.len(), string))
}

fn add_details(ie: IntermediateError, string: &str) -> PGN4Error {
    use IntermediateError::*;
    match ie {
        Other(r) => PGN4Error::Other(map_location(r, string)),
        MoveErr(m, e, r) => PGN4Error::BadMove(m, e, map_location(r, string)),
        Description(r) => PGN4Error::BadDescription(map_location(r, string)),
    }
}
