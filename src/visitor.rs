use crate::*;
use std::fmt;

/// A partially followed path.
#[derive(Clone, Debug, PartialEq)]
pub struct PartialPath<'a> {
    main: &'a [usize],
    last: usize,
}

impl fmt::Display for PartialPath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.main {
            write!(f, "{}-", i)?;
        }
        write!(f, "{}", self.last)
    }
}
impl Default for PartialPath<'_> {
    fn default() -> Self {
        Self { main: &[], last: 0 }
    }
}

impl PartialPath<'_> {
    fn done(&self, path: &[usize]) -> bool {
        if path.len() == self.main.len() + 1 && path[path.len() - 1] == self.last {
            for i in 0..self.main.len() {
                if path[i] != self.main[i] {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

use thiserror::Error;
#[derive(Error, PartialEq, Debug, Clone)]
pub enum VisitingError {
    #[error(
        "While trying to follow a step of a partial path, the partial path was wrong in some way."
    )]
    PartialPathWrong,
    #[error("Path given has an even number of numbers")]
    EvenPath,
    #[error("You tried to look one past the end of the game; stop here.")]
    UnexpectedEndOfGame,
    #[error("A zero was found in the path. Only valid way to have a zero in the path is [0].")]
    ZeroInPath,
    #[error("There was an empty turn in the pgn")]
    EmptyTurn,
    #[error(
        "Either the alternative is not present on the last qturn or a qturn has yet to be passed"
    )]
    InvalidAlternative,
    #[error("Some other internal error occured")]
    Internal,
}

/// A view into the quarterturns of a pgn4.
#[derive(Debug, Clone)]
pub struct Visitor<'a> {
    turns: &'a Vec<Turn>,
    i: usize,
    j: usize,
}

/// A mutable view into the quarterturns of a pgn4.
#[derive(Debug)]
pub struct VisitorMut<'a> {
    turns: &'a mut Vec<Turn>,
    i: usize,
    j: usize,
}

impl<'a> Visitor<'a> {
    /// Creates a new Visitor
    pub fn new(pgn4: &'a PGN4) -> Self {
        Self {
            turns: &pgn4.turns,
            i: 0,
            j: usize::MAX,
        }
    }
}

impl<'a> VisitorMut<'a> {
    /// Creates a new VisitorMut
    pub fn new(pgn4: &'a mut PGN4) -> Self {
        Self {
            turns: &mut pgn4.turns,
            i: 0,
            j: usize::MAX,
        }
    }
    /// Reborrows VisitorMut because it cannot be cloned. By keeping a trail of reborrows, it is possible to backtrack with VisitorMut
    ///
    /// TODO: example of usefullness
    pub fn reborrow<'b>(&'b mut self) -> VisitorMut<'b> {
        VisitorMut {
            turns: &mut self.turns,
            i: self.i,
            j: self.j,
        }
    }
    /// View the currently hovered QuarterTurn if it exists
    pub fn qturn_mut<'b>(&'b mut self) -> Option<&'b mut QuarterTurn> {
        if self.j == usize::MAX {
            None
        } else {
            Some(&mut self.turns[self.i].turns[self.j])
        }
    }
}

/// All common methods between Visitor and VisitorMut
pub trait VisitorCommon
where
    Self: Sized,
{
    /// Is this hovered over the last quarterturn in the line
    fn last(&self) -> bool;
    /// How many alternatives are present on the hovered quarterturn.
    fn alternatives(&self) -> usize;
    /// View the currently hovered QuarterTurn if it exists
    fn qturn(&self) -> Option<&QuarterTurn>;
    /// Move the visitor one forward
    fn next(self) -> Result<Self, VisitingError>;
    /// Move the visitor into one of the current qturns alternatives. Leaves the visitor at the start of the line without any hovered quarterturn.
    fn into_alternative(self, alt: usize) -> Result<Self, VisitingError>;
    /// Follow one step of a path.
    fn follow_once<'a>(
        mut self,
        partial: &'_ mut PartialPath<'a>,
        main: &'a [usize],
    ) -> Result<Self, VisitingError> {
        let len = partial.main.len();
        if len + 1 > main.len() {
            return Err(VisitingError::PartialPathWrong);
        }
        for i in 0..len {
            if partial.main[i] != main[i] {
                return Err(VisitingError::PartialPathWrong);
            }
        }
        let increment = partial.last != main[len];
        if partial.last > main[len] {
            return Err(VisitingError::PartialPathWrong);
        } else if increment {
            self = self.next()?;
            partial.last += 1;
        }

        if partial.last == main[len] {
            if main.len() == len + 1 {
                if !increment {
                    return Err(VisitingError::UnexpectedEndOfGame);
                }
            } else if main.len() == len + 2 {
                return Err(VisitingError::EvenPath);
            } else {
                let alt = main[len + 1];
                if main[len + 2] == 0 {
                    return Err(VisitingError::ZeroInPath);
                }
                self = self.into_alternative(alt)?.next()?; // Call both into and next to make sure `qturn()` is always valid
                partial.main = &main[0..len + 2];
                partial.last = 1;
            }
        }
        Ok(self)
    }
    /// Follow a path.
    fn follow_path<'a>(
        mut self,
        partial: &'_ mut PartialPath<'a>,
        path: &'a [usize],
    ) -> Result<Self, VisitingError> {
        if path.len() % 2 == 0 {
            return Err(VisitingError::EvenPath);
        }
        if path == &[0] {
            return Ok(self);
        }
        for val in path {
            if *val == 0 {
                return Err(VisitingError::ZeroInPath);
            }
        }
        while !partial.done(path) {
            self = self.follow_once(partial, path)?;
        }
        Ok(self)
    }
}

impl<'a> VisitorCommon for Visitor<'a> {
    fn last(&self) -> bool {
        (self.i == 0 && self.j == usize::MAX && self.turns.len() == 0)
            || (self.i == self.turns.len() - 1 && self.j == self.turns[self.i].turns.len() - 1)
    }
    fn alternatives(&self) -> usize {
        if self.j == usize::MAX {
            0
        } else {
            self.turns[self.i].turns[self.j].alternatives.len()
        }
    }
    fn qturn(&self) -> Option<&QuarterTurn> {
        if self.j == usize::MAX {
            None
        } else {
            Some(&self.turns[self.i].turns[self.j])
        }
    }

    fn next(mut self) -> Result<Self, VisitingError> {
        let i = self.i;
        let j = self.j;
        if i == 0 && j == usize::MAX {
            // special case for the beginning of line
            return if self.turns.len() == 0 {
                Err(VisitingError::UnexpectedEndOfGame)
            } else if self.turns[i].turns.len() == 0 {
                Err(VisitingError::EmptyTurn)
            } else {
                self.j = 0;
                Ok(self)
            };
        }
        let len = self.turns[i].turns.len();
        if j >= len {
            Err(VisitingError::Internal)
        } else if j + 1 == len {
            if i + 1 == self.turns.len() {
                Err(VisitingError::UnexpectedEndOfGame)
            } else if self.turns[i + 1].turns.len() == 0 {
                Err(VisitingError::EmptyTurn)
            } else {
                self.j = 0;
                self.i += 1;
                Ok(self)
            }
        } else {
            self.j += 1;
            Ok(self)
        }
    }
    fn into_alternative(self, alt: usize) -> Result<Self, VisitingError> {
        if self.j == usize::MAX {
            return Err(VisitingError::InvalidAlternative);
        }
        if alt == 0 || self.turns[self.i].turns[self.j].alternatives.len() < alt {
            return Err(VisitingError::InvalidAlternative);
        }
        Ok(Self {
            turns: &self.turns[self.i].turns[self.j].alternatives[alt - 1],
            i: 0,
            j: usize::MAX,
        })
    }
}

impl<'a> VisitorCommon for VisitorMut<'a> {
    fn last(&self) -> bool {
        (self.i == 0 && self.j == usize::MAX && self.turns.len() == 0)
            || (self.i == self.turns.len() - 1 && self.j == self.turns[self.i].turns.len() - 1)
    }
    fn alternatives(&self) -> usize {
        if self.j == usize::MAX {
            0
        } else {
            self.turns[self.i].turns[self.j].alternatives.len()
        }
    }
    fn qturn(&self) -> Option<&QuarterTurn> {
        if self.j == usize::MAX {
            None
        } else {
            Some(&self.turns[self.i].turns[self.j])
        }
    }

    fn next(mut self) -> Result<Self, VisitingError> {
        let i = self.i;
        let j = self.j;
        if i == 0 && j == usize::MAX {
            // special case for the beginning of line
            return if self.turns.len() == 0 {
                Err(VisitingError::UnexpectedEndOfGame)
            } else if self.turns[i].turns.len() == 0 {
                Err(VisitingError::EmptyTurn)
            } else {
                self.j = 0;
                Ok(self)
            };
        }
        let len = self.turns[i].turns.len();
        if j >= len {
            Err(VisitingError::Internal)
        } else if j + 1 == len {
            if i + 1 == self.turns.len() {
                Err(VisitingError::UnexpectedEndOfGame)
            } else if self.turns[i + 1].turns.len() == 0 {
                Err(VisitingError::EmptyTurn)
            } else {
                self.j = 0;
                self.i += 1;
                Ok(self)
            }
        } else {
            self.j += 1;
            Ok(self)
        }
    }
    fn into_alternative(self, alt: usize) -> Result<Self, VisitingError> {
        if self.j == usize::MAX {
            return Err(VisitingError::InvalidAlternative);
        }
        if alt == 0 || self.turns[self.i].turns[self.j].alternatives.len() < alt {
            return Err(VisitingError::InvalidAlternative);
        }
        Ok(Self {
            turns: &mut self.turns[self.i].turns[self.j].alternatives[alt - 1],
            i: 0,
            j: usize::MAX,
        })
    }
}
