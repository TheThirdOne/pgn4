const BASIC_GAME: &'static str = "1. d2-d3 .. b11-c11 .. k13-k12 .. m4-l4
2. h2-h3 .. b7-c7 .. g13-g12 .. m8-l8";

const TO_ADD: &'static str = "1. j2-j3 .. b5-c5 .. e13-e12 .. m10-l10
2. e2-e3 .. b10-c10 .. j13-j12 .. m5-l5";

const ADDED_ON_FIRST: &'static str = "1. d2-d3
(1.. j2-j3 .. b5-c5 .. e13-e12 .. m10-l10
2. e2-e3 .. b10-c10 .. j13-j12 .. m5-l5 )  .. b11-c11 .. k13-k12 .. m4-l4
2. h2-h3 .. b7-c7 .. g13-g12 .. m8-l8";

const ADDED_ON_EACH: &'static str = "1. d2-d3
(1.. j2-j3 )  .. b11-c11 ( .. b5-c5 )  .. k13-k12 ( .. e13-e12 )  .. m4-l4 ( .. m10-l10 ) 
2. h2-h3
(2.. e2-e3 )  .. b7-c7 ( .. b10-c10 )  .. g13-g12 ( .. j13-j12 )  .. m8-l8 ( .. m5-l5 )";

#[test]
fn copies_work() {
    let base: pgn4::PGN4 = BASIC_GAME.parse().unwrap();
    let copy = base.clone();
    assert_eq!(base, copy, "Clone doesn't result in the same pgn");
    let mut new = pgn4::PGN4 {
        bracketed: vec![],
        turns: vec![],
    };
    let mut ply = [0];
    for turn in copy.turns {
        for qturn in turn.turns {
            assert_eq!(
                0,
                new.append_move(&ply, qturn).unwrap(),
                "All appends should end up as 0 because there are no alternatives"
            );
            ply[0] += 1;
        }
    }
    assert_eq!(
        base, new,
        "Appending all moves doesn't result in the same pgn"
    );
    ply[0] = 0;
    let copy2 = base.clone();
    for turn in copy2.turns {
        for qturn in turn.turns {
            assert_eq!(
                0,
                new.append_move(&ply, qturn).unwrap(),
                "All appends should end up as 0 because there are no alternatives"
            );
            ply[0] += 1;
        }
    }
    println!("{} vs \n{}", base, new);
    assert_eq!(
        base, new,
        "Appending all moves a second time should be idempotent"
    );
}

#[test]
fn alternatives() {
    let mut base: pgn4::PGN4 = BASIC_GAME.parse().unwrap();
    let to_add: pgn4::PGN4 = TO_ADD.parse().unwrap();
    let mut base2: pgn4::PGN4 = BASIC_GAME.parse().unwrap();

    let added_on_first: pgn4::PGN4 = ADDED_ON_FIRST.parse().unwrap();
    let added_on_each: pgn4::PGN4 = ADDED_ON_EACH.parse().unwrap();

    let mut ply = [0];

    for turn in to_add.turns {
        for qturn in turn.turns {
            if ply[0] == 0 {
                assert_eq!(
                    1,
                    base2.append_move(&ply, qturn.clone()).unwrap(),
                    "base2-1"
                );
            } else {
                let tmp = base2.append_move(&[1, 1, ply[0]], qturn.clone()).unwrap();
                eprintln!("{}", base2);
                assert_eq!(0, tmp, "base2-2");
            }
            assert_eq!(1, base.append_move(&ply, qturn).unwrap(), "base");
            ply[0] += 1;
        }
    }
    println!("{} vs \n{}", base, added_on_each);
    assert_eq!(base, added_on_each, "added on each not right");
    assert_eq!(base2, added_on_first, "added on first not right");
}
