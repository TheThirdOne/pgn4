use crate::*;
use fen4::{Board, Color, TurnColor};
use std::num::ParseIntError;

use thiserror::Error;
#[derive(Error, PartialEq, Debug, Clone)]
pub enum VariantError {
    #[error("A bracketed value is repeated")]
    RepeatedBracket,
    #[error("Variant \"{0}\" is not recognized")]
    UnknownVariant(String),
    #[error("RuleVariant involving '=' is malformed")]
    BadEquals,
    #[error("Initial board tag failed to parse because of: {0}")]
    InvalidFen4(#[from] fen4::BoardParseError),
    #[error("RuleVariant \"{0}\" is not recognized")]
    UnknownRuleVariant(String),
    #[error("Custom position does not match what the variant would suggest")]
    MismatchedCustomPosition,
    #[error("An integer failed to parse for reason: {0}")]
    BadInt(#[from] ParseIntError),
    #[error("Some other error occured")]
    Other,
}

impl PGN4 {
    pub fn tag<'a>(&'a self, tag_name: &'_ str) -> Option<&'a str> {
        for (key, value) in &self.bracketed {
            if key == tag_name {
                return Some(&value);
            }
        }
        None
    }
    pub fn variant(&self) -> Result<Variant, VariantError> {
        use VariantError::*;
        let variant = self.tag("Variant");
        let rule_variants = self.tag("RuleVariants");
        let start_fen = self.tag("StartFen4");
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

    pub fn ratings(&self) -> Option<[u16; 4]> {
        fn tou16(s: Option<&str>) -> Option<u16> {
            s.map(|s| s.parse::<u16>().ok()).flatten()
        }
        let red = tou16(self.tag("RedElo"))?;
        let blue = tou16(self.tag("BlueElo"))?;
        let yellow = tou16(self.tag("YellowElo"))?;
        let green = tou16(self.tag("GreenElo"))?;
        Some([red, blue, yellow, green])
    }

    pub fn players(&self) -> Option<[&str; 4]> {
        let red = self.tag("Red")?;
        let blue = self.tag("Blue")?;
        let yellow = self.tag("Yellow")?;
        let green = self.tag("Green")?;
        Some([red, blue, yellow, green])
    }

    pub fn result(&self) -> GameResult {
        use GameResult::*;
        let result = self.tag("Result");
        let red_name = self.tag("Red");
        let blue_name = self.tag("Blue");
        let yellow_name = self.tag("Yellow");
        let green_name = self.tag("Green");
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
    pub fn append_move(&mut self, path: &[usize], q: QuarterTurn) -> Result<usize, ()> {
        if path.len() % 2 == 0 {
            // Format for path is [ply forward] : [alternative index, plyforward] * so it must be odd
            eprintln!("Invalid path length");
            return Err(());
        }
        if path.len() == 1 && path[0] == 0 && self.turns.len() == 0 {
            self.turns.push(Turn {
                number: 1,
                double_dot: false,
                turns: vec![q],
            });
            return Ok(0);
        }
        fn helper(
            turns: &mut Vec<Turn>,
            path: &[usize],
            q: QuarterTurn,
            mut total: usize,
        ) -> Result<usize, ()> {
            let ply = path[0];
            if ply == 0 && path.len() > 2 {
                eprintln!("E1");
                return Err(());
            }
            let mut current = 0;
            for turn in turns.iter_mut() {
                for qturn in &mut turn.turns {
                    current += 1;
                    if path.len() > 2 {
                        if current == ply {
                            let alt = path[1];
                            if alt == 0 || qturn.alternatives.len() < alt {
                                eprintln!("E2");
                                return Err(());
                            } else {
                                return helper(
                                    &mut qturn.alternatives[alt - 1],
                                    &path[2..],
                                    q,
                                    total,
                                );
                            }
                        }
                    } else if ply == current - 1 {
                        if qturn.main == q.main && qturn.modifier == q.modifier {
                            return Ok(0);
                        } else {
                            let mut i = 0;
                            for alt in &qturn.alternatives {
                                i += 1;
                                let tmp = &alt[0].turns[0];
                                if tmp.main == q.main && tmp.modifier == q.modifier {
                                    return Ok(i);
                                }
                            }
                            let number = if total % 4 == 0 { total / 4 + 1 } else { 0 };
                            qturn.alternatives.push(vec![Turn {
                                number,
                                double_dot: true,
                                turns: vec![q],
                            }]);
                            return Ok(i + 1);
                        }
                    }
                    total += 1;
                }
            }
            if ply == current {
                // Use total % 4 for now to determine if a new turn is needed
                // In the future use alivecount
                let last = turns.len() - 1;
                if total % 4 == 0 {
                    let number = total / 4 + 1;
                    let double_dot = false; //turns[0].double_dot;
                    turns.push(Turn {
                        number,
                        double_dot,
                        turns: vec![q],
                    });
                } else {
                    turns[last].turns.push(q);
                }
                return Ok(0);
            } else {
                return Err(());
            }
        }
        helper(&mut self.turns, path, q, 0)
    }
    pub fn promote_to_mainline(&mut self, path: &[usize]) -> Result<(), ()> {
        if path.len() % 2 == 0 {
            return Err(());
        }
        fn helper(turns: &mut Vec<Turn>, path: &[usize]) -> Result<(), ()> {
            let ply = path[0];
            if ply == 0 {
                return Err(());
            }
            if path.len() == 1 {
                return Ok(());
            }
            let mut current = 0;
            let t_len = turns.len();
            for i in 0..t_len {
                let q_len = turns[i].turns.len();
                for j in 0..q_len {
                    current += 1;
                    if current == ply {
                        let alt = path[1];
                        let mut to_mainline = {
                            let alternatives = &mut turns[i].turns[j].alternatives;
                            if alt == 0 || alternatives.len() < alt {
                                eprintln!("E2");
                                return Err(());
                            } else {
                                helper(&mut alternatives[alt - 1], &path[2..])?;
                                alternatives.remove(alt - 1)
                            }
                        };
                        // Start by moving over alternatives before it becomes harder
                        std::mem::swap(
                            &mut to_mainline[0].turns[0].alternatives,
                            &mut turns[i].turns[j].alternatives,
                        );

                        // Break turns => turns, turn, to_alternate
                        let mut to_alternate = turns.split_off(i);
                        let mut beginning_halfturn = to_alternate[0].turns.split_off(j);
                        std::mem::swap(&mut beginning_halfturn, &mut to_alternate[0].turns);
                        let mut turn = Turn {
                            number: to_alternate[0].number,
                            double_dot: to_alternate[0].double_dot,
                            turns: beginning_halfturn,
                        };

                        // Complete swap of number and doubledot
                        to_alternate[0].number = to_mainline[0].number;
                        to_alternate[0].double_dot = to_mainline[0].double_dot;

                        // to_alternate is now complete, add it to the front of alternatives
                        to_mainline[0].turns[0].alternatives.insert(0, to_alternate);

                        // Prepend turn into to_mainline
                        turn.turns.append(&mut to_mainline[0].turns);
                        std::mem::swap(&mut turn, &mut to_mainline[0]);

                        // Add to_mainline back into turn
                        turns.append(&mut to_mainline);
                        return Ok(());
                    }
                }
            }
            return Err(());
        }
        helper(&mut self.turns, path)
    }
    pub fn delete_from(&mut self, path: &[usize]) -> Result<(), ()> {
        if path.len() % 2 == 0 {
            return Err(());
        }
        fn helper(turns: &mut Vec<Turn>, path: &[usize]) -> Result<(), ()> {
            let ply = path[0];
            if ply == 0 {
                return Err(());
            }
            let mut current = 0;
            let t_len = turns.len();
            for i in 0..t_len {
                let q_len = turns[i].turns.len();
                for j in 0..q_len {
                    current += 1;
                    if current == ply {
                        if path.len() > 2 {
                            let alt = path[1];
                            let alternatives = &mut turns[i].turns[j].alternatives;
                            if alt == 0 || alternatives.len() < alt {
                                return Err(());
                            } else {
                                helper(&mut alternatives[alt - 1], &path[2..])?;
                                // If the deletion makes an alternative empty, delete it
                                if alternatives[alt - 1].len() == 0 {
                                    alternatives.remove(alt - 1);
                                }
                            }
                        } else {
                            // Start by deleting unneeded qturns
                            turns.truncate(i + 1);
                            turns[i].turns.truncate(j + 1);

                            if turns[i].turns[j].alternatives.len() != 0 {
                                // If it has an alternative, promote the first one
                                let mut to_append = turns[i].turns[j].alternatives.remove(0);
                                let mut first_turn = to_append.remove(0);

                                // Swap over any remaining alternatives
                                std::mem::swap(
                                    &mut first_turn.turns[0].alternatives,
                                    &mut turns[i].turns[j].alternatives,
                                );

                                // Add moves onto the current turn (replacing the deleted move)
                                turns[i].turns.pop();
                                turns[i].turns.append(&mut first_turn.turns);

                                // Add the rest of the turns
                                turns.append(&mut to_append);
                            } else {
                                // If no alternatives, just delete it and if it is the only qturn in a turn delete that too.
                                turns[i].turns.pop();
                                if turns[i].turns.len() == 0 {
                                    turns.pop();
                                }
                            }
                        }
                        return Ok(());
                    }
                }
            }
            return Err(());
        }
        helper(&mut self.turns, path)
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
