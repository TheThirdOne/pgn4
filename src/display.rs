use std::fmt;

use crate::types::*;

impl fmt::Display for BasicMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.piece != 'P' {
            write!(f, "{}", self.piece)?;
        }
        write!(
            f,
            "{}{}",
            self.from,
            if self.captured.is_some() { "x" } else { "-" },
        )?;
        if let Some(p) = self.captured {
            if p != 'P' {
                write!(f, "{}", p)?;
            }
        }
        write!(f, "{}", self.to)?;
        if let Some(p) = self.promotion {
            write!(f, "={}", p)?;
        }
        for _ in 0..self.checks {
            write!(f, "+")?;
        }
        for _ in 0..self.mates {
            write!(f, "#")?;
        }
        Ok(())
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Move::*;
        write!(
            f,
            "{}",
            match self {
                Checkmate => "#",
                Stalemate => "S",
                Timeout => "T",
                Resign => "R",
                KingCastle(s) | QueenCastle(s) => {
                    write!(f, "O-O")?;
                    if let QueenCastle(_) = self {
                        write!(f, "-O")?;
                    }
                    for _ in 0..*s {
                        write!(f, "#")?;
                    }
                    return Ok(());
                }
                Normal(bm) => return write!(f, "{}", bm),
            }
        )
    }
}

struct MoveHelper<'a>(&'a Move);
impl fmt::Debug for MoveHelper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Move").field(&self.0.to_string()).finish()
    }
}

impl fmt::Debug for QuarterTurn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = f.debug_struct("QuarterTurn");
        out.field("main", &MoveHelper(&self.main));
        if let Some(m) = &self.modifier {
            out.field("modifier", &MoveHelper(m));
        }
        if self.extra_stalemate {
            out.field("extra_stalemate", &true);
        }
        if let Some(d) = &self.description {
            out.field("description", d);
        }
        if self.alternatives.len() != 0 {
            out.field("alternatives", &self.alternatives);
        }
        out.finish()
    }
}
impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.number != 0 {
            write!(f, "{}", self.number)?;
        }
        if self.double_dot {
            write!(f, ".. ")?;
        } else {
            write!(f, ". ")?;
        }
        write!(f, "{}", self.turns[0])?;
        for quarter in &self.turns[1..] {
            write!(f, " .. {}", quarter)?;
        }
        Ok(())
    }
}

impl fmt::Display for QuarterTurn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.main)?;
        if let Some(modifier) = &self.modifier {
            write!(f, "{}", modifier)?;
        }
        if self.extra_stalemate {
            write!(f, "S")?;
        }
        if let Some(d) = &self.description {
            write!(f, " {{ {} }}", d)?;
        }
        for alt in &self.alternatives {
            // Just ignore it if len is 0 and it is for some reason instantiated.
            if alt.len() != 0 {
                // The initial '(' has different whitespace if the alternative starts
                // in the middle of a turn version the beginning
                if alt[0].number == 0 {
                    write!(f, " ( ")?;
                } else {
                    write!(f, "\n(")?;
                }
                // Write \n separated turns
                write!(f, "{}", alt[0])?;
                for turn in &alt[1..] {
                    write!(f, "\n{}", turn)?;
                }
                // The ending always has space on both sides
                write!(f, " ) ")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for PGN4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.bracketed.len() != 0 {
            for bracket in &self.bracketed {
                write!(f, "[{} \"{}\"]\n", bracket.0, bracket.1)?;
            }
            write!(f, "\n\n\n")?;
        }
        if self.turns.len() != 0 {
            write!(f, "{}", self.turns[0])?;
            for turn in &self.turns[1..] {
                write!(f, "\n{}", turn)?;
            }
        }
        Ok(())
    }
}
