use crate::*;
use fen4::{Board, Color, TurnColor};
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub enum VariantError {
    RepeatedBracket,
    UnknownVariant(String),
    BadEquals,
    InvalidFen4(fen4::BoardParseError),
    UnknownRuleVariant(String),
    MismatchedCustomPosition,
    BadInt(ParseIntError),
    Other,
}

impl PGN4 {
    pub fn variant(&self) -> Result<Variant, VariantError> {
        use VariantError::*;
        let mut variant: Option<&str> = None;
        let mut rule_variants: Option<&str> = None;
        let mut start_fen: Option<&str> = None;
        for (key, value) in &self.bracketed {
            match key.as_ref() {
                "Variant" => {
                    if variant.is_some() {
                        return Err(RepeatedBracket);
                    }
                    variant = Some(&value);
                }
                "RuleVariants" => {
                    if rule_variants.is_some() {
                        return Err(RepeatedBracket);
                    }
                    rule_variants = Some(&value);
                }
                "StartFen4" => {
                    if start_fen.is_some() {
                        return Err(RepeatedBracket);
                    }
                    start_fen = Some(&value);
                }
                _ => {}
            }
        }
        let mut base = match variant.unwrap_or("") {
            "Solo" | "FFA" => Variant::ffa_default(),
            "Teams" => Variant::team_default(),
            s => return Err(UnknownVariant(s.into())),
        };

        let custom_board = start_fen.is_some();
        if let Some(fen) = start_fen {
            base.initial_board = fen.parse().map_err(|e| {
                eprintln!("failed to parse fen4 because or error {:?}", e);
                InvalidFen4(e)
            })?;
        }
        if let Some(tmp) = rule_variants {
            for rule in tmp.split(' ') {
                if rule.contains('=') {
                    let mut iter = rule.split("=");
                    let key = iter.next().unwrap();
                    let value = iter.next().unwrap();
                    if iter.next().is_some() {
                        return Err(BadEquals);
                    }
                    if key == "PromoteTo" {
                        base.promote_to = value.chars().collect();
                    } else if key == "Chess960" {
                        let int_value: u16 = value.parse().map_err(|ie| BadInt(ie))?;
                        base.chess960 = int_value;
                        let new_board = Board::chess960(int_value);
                        if custom_board && base.initial_board.board != new_board.board {
                            return Err(MismatchedCustomPosition);
                        }
                        base.initial_board.board = new_board.board;
                    } else {
                        let int_value: u8 = value.parse().map_err(|ie| BadInt(ie))?;
                        match key {
                            "PointsForMate" => base.ffa_points_for_mate = int_value.into(),
                            "Prom" => base.pawn_promotion_rank = int_value.into(),
                            "OppX" => base.ffa_opp_x = int_value.into(),
                            "Teammate" => {
                                base.red_teammate = match int_value {
                                    1 => Color::Turn(TurnColor::Blue),
                                    2 => Color::Turn(TurnColor::Yellow),
                                    3 => Color::Turn(TurnColor::Green),
                                    _ => return Err(Other),
                                }
                            }
                            _ => return Err(UnknownRuleVariant(key.into())),
                        }
                    }
                } else if rule.ends_with("-check") {
                    let mut iter = rule.split('-');
                    let count = iter.next().unwrap();
                    let int_count: usize = count.parse().map_err(|ie| BadInt(ie))?;
                    base.ncheck = int_count;
                    // TODO: reject if not None or equal
                    base.initial_board.extra_options.lives = Some([int_count; 4]);
                } else {
                    *(match rule {
                        "EnPassant" => &mut base.en_passant,
                        "KotH" => &mut base.king_of_the_hill,
                        "DeadWall" => &mut base.dead_wall,
                        "CaptureTheKing" => &mut base.capture_the_king,
                        "Antichess" => &mut base.antichess,
                        "DeadKingWalking" => &mut base.ffa_dead_king_walking,
                        "Play-4-Mate" => &mut base.ffa_play_for_mate,
                        "Takeover" => &mut base.ffa_takeover,
                        "Anonymous" | "Ghostboard" | "SpectatorChat" | "Diplomacy"
                        | "Blindfold" => continue,
                        _ => return Err(UnknownRuleVariant(rule.into())),
                        //_ => continue,
                    }) = true;
                }
            }
        }
        Ok(base)
    }
    pub fn result(&self) -> GameResult {
        use GameResult::*;
        let mut result: Option<&str> = None;
        let mut red_name: Option<&str> = None;
        let mut blue_name: Option<&str> = None;
        let mut yellow_name: Option<&str> = None;
        let mut green_name: Option<&str> = None;
        for (key, value) in &self.bracketed {
            match key.as_ref() {
                "Result" => {
                    if result.is_some() {
                        return Error;
                    }
                    result = Some(&value);
                }
                "Red" => {
                    if red_name.is_some() {
                        return Error;
                    }
                    red_name = Some(&value);
                }
                "Blue" => {
                    if blue_name.is_some() {
                        return Error;
                    }
                    blue_name = Some(&value);
                }
                "Yellow" => {
                    if yellow_name.is_some() {
                        return Error;
                    }
                    yellow_name = Some(&value);
                }
                "Green" => {
                    if green_name.is_some() {
                        return Error;
                    }
                    green_name = Some(&value);
                }
                _ => {}
            }
        }
        if let Some(r) = result {
            if r == "Aborted" {
                Aborted
            } else if r == "Draw" {
                Team(false, false)
            } else if r == "1-0" {
                Team(true, false)
            } else if r == "0-1" {
                Team(false, true)
            } else {
                let colors = [red_name, blue_name, yellow_name, green_name];
                let mut scores = [0; 4];
                let mut segments = r.split(" - ");
                for i in 0..4 {
                    if let Some(color_name) = colors[i] {
                        let segment = match segments.next() {
                            Some(s) => s.trim(),
                            None => return Error,
                        };
                        let mut iter = segment.split(':');
                        let name = match iter.next() {
                            Some(s) => s.trim(),
                            None => return Error,
                        };
                        let score = match iter.next() {
                            Some(s) => s.trim(),
                            None => return Error,
                        };
                        if iter.next().is_some() {
                            return Error;
                        }
                        if color_name != name {
                            return Error;
                        }
                        scores[i] = match score.parse() {
                            Ok(int) => int,
                            Err(_) => return Error,
                        };
                    }
                }
                if segments.next().is_some() {
                    return Error;
                }
                FFA(scores)
            }
        } else {
            Error
        }
    }
}

impl Variant {
    pub fn team_default() -> Self {
        Self {
            red_teammate: Color::Turn(TurnColor::Yellow),

            initial_board: Board::default(),
            king_of_the_hill: false,
            antichess: false,
            promote_to: vec!['Q', 'R', 'B', 'N'],
            dead_wall: false,
            en_passant: false,
            capture_the_king: false,
            pawn_promotion_rank: 11,
            ncheck: 0,
            chess960: 0,

            ffa_dead_king_walking: false,
            ffa_takeover: false,
            ffa_opp_x: 0,
            ffa_points_for_mate: 0,
            ffa_play_for_mate: false,
        }
    }
    pub fn ffa_default() -> Self {
        Self {
            red_teammate: Color::Dead(None),

            initial_board: Board::default(),
            king_of_the_hill: false,
            antichess: false,
            promote_to: vec!['D'],
            dead_wall: false,
            en_passant: false,
            capture_the_king: false,
            pawn_promotion_rank: 8,
            ncheck: 0,
            chess960: 0,

            ffa_dead_king_walking: false,
            ffa_takeover: false,
            ffa_opp_x: 1,
            ffa_points_for_mate: 20,
            ffa_play_for_mate: false,
        }
    }
    pub fn fairy(&self) -> bool {
        let normal = ['P', 'B', 'N', 'R', 'Q', 'D', 'K'];
        for i in 0..14 {
            for j in 0..14 {
                if let fen4::Piece::Normal(_, c) = self.initial_board.board[i][j] {
                    if !normal.contains(&c) {
                        return true;
                    }
                }
            }
        }
        for c in &self.promote_to {
            if !normal.contains(c) {
                return true;
            }
        }
        false
    }
    pub fn pawn_base_rank(&self) -> usize {
        self.initial_board.extra_options.pawnbaserank
    }
}

use std::fmt;
impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let default = if let Color::Turn(c) = self.red_teammate {
            write!(f, "TEAM - ")?;
            if c != TurnColor::Yellow {
                let int_value: usize = c.into();
                write!(f, "Teammate={} ", int_value)?;
            }
            Self::team_default()
        } else {
            write!(f, "FFA - ")?;
            Self::ffa_default()
        };
        if self.initial_board != default.initial_board {
            write!(f, "CUSTOM_POSITION ")?;
            if self.fairy() {
                write!(f, "FAIRY ")?;
            }
        }
        if self.king_of_the_hill {
            write!(f, "KotH ")?;
        }
        if self.antichess {
            write!(f, "Antichess ")?;
        }
        if self.promote_to != default.promote_to {
            write!(f, "PromoteTo=")?;
            for c in &self.promote_to {
                write!(f, "{}", c)?;
            }
            write!(f, " ")?;
        }
        if self.chess960 != default.chess960 {
            write!(f, "Chess960={} ", self.chess960)?;
        }
        if self.dead_wall {
            write!(f, "DeadWall ")?;
        }
        if self.en_passant {
            write!(f, "EnPassant ")?;
        }
        if self.capture_the_king {
            write!(f, "CaptureTheKing ")?;
        }
        if self.pawn_promotion_rank != default.pawn_promotion_rank {
            write!(f, "Prom={} ", self.pawn_promotion_rank)?;
        }
        if self.ncheck != default.ncheck {
            write!(f, "{}-check ", self.ncheck)?;
        }
        if self.ffa_dead_king_walking {
            write!(f, "DeadKingWalking ")?;
        }
        if self.ffa_takeover {
            write!(f, "Takeover ")?;
        }
        if self.ffa_opp_x != default.ffa_opp_x {
            write!(f, "OppX={} ", self.ffa_opp_x)?;
        }
        if self.ffa_points_for_mate != default.ffa_points_for_mate {
            write!(f, "PointsForMate={} ", self.ffa_points_for_mate)?;
        }
        if self.ffa_play_for_mate {
            write!(f, "Play-4-Mate ")?;
        }
        Ok(())
    }
}
