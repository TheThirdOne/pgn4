use crate::*;

impl From<Move> for QuarterTurn {
    fn from(main: Move) -> Self {
        Self {
            main,
            modifier: None,
            description: None,
            alternatives: Vec::new(),
            extra_stalemate: false,
        }
    }
}
