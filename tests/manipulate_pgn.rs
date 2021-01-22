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

fn add_each(work: &mut pgn4::PGN4, to_add: &pgn4::PGN4) {
    let mut ply = 0;
    for turn in &to_add.turns {
        for qturn in &turn.turns {
            work.append_move(&[ply], qturn.clone()).unwrap();
            ply += 1;
        }
    }
}

fn add_first(work: &mut pgn4::PGN4, to_add: &pgn4::PGN4) {
    let mut ply = 0;
    for turn in &to_add.turns {
        for qturn in &turn.turns {
            if ply == 0 {
                work.append_move(&[0], qturn.clone()).unwrap();
            } else {
                work.append_move(&[1, 1, ply], qturn.clone()).unwrap();
            }
            ply += 1;
        }
    }
}

#[test]
fn copies_work() {
    let base: pgn4::PGN4 = BASIC_GAME.parse().unwrap();
    let mut new = pgn4::PGN4 {
        bracketed: vec![],
        turns: vec![],
    };
    add_each(&mut new, &base);
    assert_eq!(
        base, new,
        "Appending all moves doesn't result in the same pgn"
    );
    add_each(&mut new, &base);
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

    add_each(&mut base, &to_add);
    add_first(&mut base2, &to_add);

    assert_eq!(base, added_on_each, "added on each not right");
    assert_eq!(base2, added_on_first, "added on first not right");

    base2.promote_to_mainline(&[1, 1, 1]).unwrap();
    assert_ne!(base2, added_on_first, "Promotion didn't change pgn");
    base2.promote_to_mainline(&[1, 1, 1]).unwrap();
    assert_eq!(
        base2, added_on_first,
        "Promotion with two possiblities is its own inverse"
    );

    base.promote_to_mainline(&[2, 1, 1]).unwrap();
    assert_ne!(base, added_on_each, "Promotion didn't change pgn 2");
    base.promote_to_mainline(&[2, 1, 1]).unwrap();
    assert_eq!(
        base, added_on_each,
        "Promotion with two possiblities is its own inverse 2"
    );

    base.promote_to_mainline(&[4, 1, 1]).unwrap();
    assert_ne!(base, added_on_each, "Promotion didn't change pgn 3");
    base.promote_to_mainline(&[4, 1, 1]).unwrap();
    assert_eq!(
        base, added_on_each,
        "Promotion with two possiblities is its own inverse 3"
    );
}

#[test]
fn deletion() {
    let base: pgn4::PGN4 = BASIC_GAME.parse().unwrap();
    let to_add: pgn4::PGN4 = TO_ADD.parse().unwrap();
    let added_on_first: pgn4::PGN4 = ADDED_ON_FIRST.parse().unwrap();
    let added_on_each: pgn4::PGN4 = ADDED_ON_EACH.parse().unwrap();
    let empty = pgn4::PGN4 {
        bracketed: vec![],
        turns: vec![],
    };

    let mut tmp = added_on_first.clone();
    tmp.delete_from(&[1]).unwrap();
    assert_eq!(
        tmp, to_add,
        "Removing the mainline should leave you with the first variation"
    );

    let mut tmp = added_on_first.clone();
    tmp.delete_from(&[1, 1, 1]).unwrap();
    assert_eq!(
        tmp, base,
        "Removing the variation should leave you with the mainline"
    );
    tmp.delete_from(&[1]).unwrap();
    assert_eq!(
        tmp, empty,
        "Removing the variation and the mainline should leave you with nothing"
    );

    let mut tmp = added_on_each.clone();
    for i in 1..9 {
        tmp.delete_from(&[i, 1, 1]).unwrap();
    }
    assert_eq!(
        tmp, base,
        "Removing every variation should leave you with the mainline"
    );

    let mut tmp = added_on_first.clone();
    tmp.promote_to_mainline(&[1, 1, 1]).unwrap();
    tmp.delete_from(&[1]).unwrap();
    assert_eq!(
        tmp, base,
        "Promoting the variation and deleting the mainline should leave you with the mainline"
    );
}

// TODO: test deletion of nested and multiple alternatives
