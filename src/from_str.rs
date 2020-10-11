use std::convert::TryInto;
use std::str::FromStr;

use crate::types::*;

#[derive(PartialEq, Clone, Debug)]
pub struct BadPosition;
impl FromStr for Position {
    type Err = BadPosition;
    fn from_str(small: &str) -> Result<Self, Self::Err> {
        let mut iter = small.chars();
        let column_letter = iter.next().ok_or(BadPosition)?;
        if column_letter > 'n' || column_letter < 'a' {
            return Err(BadPosition);
        }

        let a: u32 = 'a'.into();
        let mut column_num: u32 = column_letter.into();
        column_num -= a;
        let col: usize = column_num.try_into().map_err(|_| BadPosition)?;

        let number_str = iter.as_str();
        let row = number_str.parse::<usize>().map_err(|_| BadPosition)?;
        if row == 0 {
            return Err(BadPosition);
        }
        Ok(Position { col, row: row - 1 })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BadBasicMove;
impl FromStr for BasicMove {
    type Err = BadBasicMove;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut iter = string.chars();
        let start = iter.next().ok_or(BadBasicMove)?;
        let (piece, pieceless) = if start.is_ascii_lowercase() {
            ('P', string)
        } else {
            (start, iter.as_str())
        };
        let mateless = pieceless.trim_end_matches('#');
        let checkless = mateless.trim_end_matches('+');

        let mates = pieceless.len() - mateless.len();
        let checks = mateless.len() - checkless.len();

        let loc = if let Some(dash) = checkless.find('-') {
            dash
        } else if let Some(x) = checkless.find('x') {
            x
        } else {
            return Err(BadBasicMove);
        };
        let (left, tmp) = checkless.split_at(loc);
        let (mid, mut right) = tmp.split_at(1); // x and - are both ascii and therefore 1 byte
        let from = left.parse::<Position>().map_err(|_| BadBasicMove)?;
        let captured = if mid == "x" {
            let mut iter = right.chars();
            let start = iter.next().ok_or(BadBasicMove)?;
            Some(if start.is_ascii_lowercase() {
                'P'
            } else {
                right = iter.as_str();
                start
            })
        } else {
            None
        };
        let to = right.parse::<Position>().map_err(|_| BadBasicMove)?;
        Ok(BasicMove {
            piece,
            from,
            captured,
            to,
            checks,
            mates,
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BadMove;
impl FromStr for Move {
    type Err = BadMove;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use Move::*;
        Ok(match string {
            "#" => Checkmate,
            "S" => Stalemate,
            "T" => Timeout,
            "R" => Resign,
            _ => Normal(string.parse::<BasicMove>().map_err(|_| BadMove)?),
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BadQuarterTurn;
fn parse_quarter(string: &str) -> Result<(QuarterTurn, &str), BadQuarterTurn> {
    /// The end of the main Move is bounded by the opening brace of a description
    /// or alternative, the first '.' of a "..", a beginning of a new turn (number),
    ///  ')' that closes the currentTurn, or EOF
    fn next_move(c: char) -> bool {
        c.is_whitespace()
            || match c {
                '.' | '{' | '(' | ')' => true,
                _ => false,
            }
    }
    let trimmed = string.trim_start();
    if trimmed == "" {
        return Err(BadQuarterTurn);
    }
    let split = trimmed.find(next_move).unwrap_or(string.len() - 1);
    let (main_str, mut rest) = trimmed.split_at(split);
    let main = main_str.trim().parse().map_err(|_| BadQuarterTurn)?;
    let mut description = None;
    let mut alternatives = Vec::new();
    rest = rest.trim_start();

    if let Some(c) = rest.chars().next() {
        if c == '{' {
            let desc_end = rest.find('}').ok_or(BadQuarterTurn)?;
            let (mut desc_str, rest_tmp) = rest.split_at(desc_end + 1);
            rest = rest_tmp;
            desc_str = desc_str.strip_prefix("{ ").ok_or(BadQuarterTurn)?;
            desc_str = desc_str.strip_suffix(" }").ok_or(BadQuarterTurn)?;
            description = Some(desc_str.to_owned());
        }
    } else {
        return Ok((
            QuarterTurn {
                main,
                description,
                alternatives,
            },
            rest,
        ));
    };
    while let Some(c) = rest.chars().next() {
        if c == '(' {
            let mut turns = Vec::new();
            rest = rest.strip_prefix('(').ok_or(BadQuarterTurn)?;
            while rest.chars().next() != Some(')') {
                let (turn, rest_tmp) = parse_turn(rest).map_err(|_| BadQuarterTurn)?;
                rest = rest_tmp;
                turns.push(turn);
            }
            rest = rest.strip_prefix(')').ok_or(BadQuarterTurn)?.trim_start();
            alternatives.push(turns);
        } else {
            break;
        }
    }
    Ok((
        QuarterTurn {
            main,
            description,
            alternatives,
        },
        rest,
    ))
}

#[derive(PartialEq, Clone, Debug)]
pub struct BadTurn;
fn parse_turn(string: &str) -> Result<(Turn, &str), BadTurn> {
    let trimmed = string.trim_start();
    let dot_loc = trimmed.find('.').ok_or(BadTurn)?;
    let (number_str, dots) = trimmed.split_at(dot_loc);
    let number = if number_str == "" {
        0
    } else {
        number_str.parse().map_err(|_| BadTurn)?
    };
    let dot = dots.strip_prefix('.').unwrap();
    let (mut rest, double_dot) = if let Some(dotted) = dot.strip_prefix('.') {
        (dotted, true)
    } else {
        (dot, false)
    };
    let mut turns = Vec::new();
    let (qturn, rest_tmp) = parse_quarter(rest).map_err(|_| BadTurn)?;
    rest = rest_tmp.trim_start();
    turns.push(qturn);
    while let Some(rest_tmp) = rest.strip_prefix("..") {
        if turns.len() >= 4 {
            return Err(BadTurn);
        }
        let (qturn, rest_tmp) = parse_quarter(rest_tmp).map_err(|_| BadTurn)?;
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

#[derive(PartialEq, Clone, Debug)]
pub struct BadPGN4;
impl FromStr for PGN4 {
    type Err = BadPGN4;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut bracketed = Vec::new();
        let mut rest = string;
        while let Some(rest_tmp) = rest.strip_prefix('[') {
            let label_end = rest_tmp.find(|c: char| c.is_whitespace()).unwrap_or(0);
            let (label, middle) = rest_tmp.split_at(label_end);
            rest = middle.trim_start().strip_prefix('"').ok_or(BadPGN4)?;

            let value_end = rest.find('"').ok_or(BadPGN4)?;
            let (value, end) = rest.split_at(value_end);
            rest = end.strip_prefix("\"]").ok_or(BadPGN4)?.trim_start();

            bracketed.push((label.to_owned(), value.to_owned()));
        }
        let mut turns = Vec::new();
        while rest != "" {
            let (turn, rest_tmp) = parse_turn(rest).map_err(|_| BadPGN4)?;
            rest = rest_tmp;
            turns.push(turn);
        }
        Ok(PGN4 { bracketed, turns })
    }
}
