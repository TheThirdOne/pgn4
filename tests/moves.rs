use pgn4::Move;

fn check(moves: &[&str]) {
    let mut failed = false;
    for move_str in moves {
        let parsed: Move = match move_str.parse() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to parse \"{}\" with error {:?}", move_str, e);
                failed = true;
                continue;
            }
        };
        let string = parsed.to_string();
        if string != *move_str {
            eprintln!("\"{}\" => Move => \"{}\"", move_str, string);
            failed = true;
        }
    }
    if failed {
        panic!("At least move failed to parse correctly");
    }
}

#[test]
fn normal() {
    let moves = vec![
        "i2-i3",
        "Na5-c6",
        "Kh1-i2",
        "Bn9xj13",
        "Ba6xBb7",
        "Bj13xRk14",
        "Qg1-h1",
        "Ra4-a6",
    ];
    check(&moves);
}

#[test]
fn castling() {
    let moves = vec!["O-O", "O-O-O"];
    check(&moves);
}

#[test]
fn checks() {
    let moves = vec!["Qi8-h9", "Qi8-h9+", "Qi8-h9++", "Qi8-h9+++", "h2-h3++"];
    //let should_fail = vec!["O-O+"];
    check(&moves);
}

#[test]
fn mates() {
    let moves = vec!["#", "Qi8-i12#", "Qi8-h9+++##", "O-O#", "O-O##", "O-O-O##"];
    //let should_fail = vec!["##"];
    check(&moves);
}

#[test]
fn basic_extra() {
    let moves = vec!["S", "T", "R"];
    check(&moves);
}
