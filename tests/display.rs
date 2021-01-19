use pgn4::*;

#[test]
fn opening_diff() {
    let expected = "1. h2-h3 .. b7-c7 .. g13-g12 ( .. h13-h12 )  .. Nn10-l9 { Test }";
    let mut moves: Vec<QuarterTurn> = vec!["h2-h3", "b7-c7", "g13-g12", "Nn10-l9"]
        .into_iter()
        .map(|s| QuarterTurn {
            main: Move::Normal(s.parse().unwrap()),
            modifier: None,
            extra_stalemate: false,
            description: None,
            alternatives: Vec::new(),
        })
        .collect();
    let insert: BasicMove = "h13-h12".parse().unwrap();
    let insert_turn = Turn {
        number: 0,
        double_dot: true,
        turns: vec![QuarterTurn {
            main: Move::Normal(insert),
            modifier: None,
            extra_stalemate: false,
            description: None,
            alternatives: Vec::new(),
        }],
    };
    moves[2].alternatives = vec![vec![insert_turn]];
    moves[3].description = Some("Test".to_owned());
    let turn = Turn {
        number: 1,
        double_dot: false,
        turns: moves,
    };
    let string = turn.to_string();
    assert_eq!(expected, string, "Simple Turn doesn't display correctly");
}

#[test]
fn full_turn_diff() {
    let expected =
        "1. h2-h3\n(1.. Nj1-i3 .. Na5-c6 .. Ne14-f12 .. m8-l8 )  .. b7-c7 .. g13-g12 .. Nn10-l9";
    let mut main_moves: Vec<QuarterTurn> = vec!["h2-h3", "b7-c7", "g13-g12", "Nn10-l9"]
        .into_iter()
        .map(|s| QuarterTurn {
            main: Move::Normal(s.parse().unwrap()),
            modifier: None,
            extra_stalemate: false,
            description: None,
            alternatives: Vec::new(),
        })
        .collect();
    let alt_moves: Vec<QuarterTurn> = vec!["Nj1-i3", "Na5-c6", "Ne14-f12", "m8-l8"]
        .into_iter()
        .map(|s| QuarterTurn {
            main: Move::Normal(s.parse().unwrap()),
            modifier: None,
            extra_stalemate: false,
            description: None,
            alternatives: Vec::new(),
        })
        .collect();
    let insert_turn = Turn {
        number: 1,
        double_dot: true,
        turns: alt_moves,
    };
    main_moves[0].alternatives = vec![vec![insert_turn]];
    let turn = Turn {
        number: 1,
        double_dot: false,
        turns: main_moves,
    };
    let string = turn.to_string();
    assert_eq!(expected, string, "Full turn diff doesn't display correctly");
}
