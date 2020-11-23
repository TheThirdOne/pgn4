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
                        // possibly verify board state
                    } else {
                        let int_value: u8 = value.parse().map_err(|ie| BadInt(ie))?;
                        match key {
                            "PointsForMate" => base.ffa_points_for_mate = int_value.into(),
                            "Prom" => base.pawn_promotion_rank = int_value.into(),
                            "OppX" => base.ffa_opp_x = int_value.into(),
                            "Teammate" => {
                                base.red_teammate = match int_value {
                                    1 => Color::Turn(TurnColor::Green),
                                    2 => Color::Turn(TurnColor::Yellow),
                                    3 => Color::Turn(TurnColor::Blue),
                                    _ => return Err(Other),
                                }
                            }
                            _ => return Err(UnknownRuleVariant(key.into())),
                        }
                    }
                } else if rule.ends_with("check") {
                    // N-check
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

            ffa_dead_king_walking: false,
            ffa_takeover: false,
            ffa_opp_x: 1,
            ffa_points_for_mate: 20,
            ffa_play_for_mate: false,
        }
    }
}
