use pgn4::*;
#[test]
fn simple_full_parse() {
    let examples =
        vec!["1. h2-h3 .. b7-c7 .. g13-g12 ( .. h13-h12 )  .. Nn10-l9 { Test }",
     "1. h2-h3\n(1.. Nj1-i3 .. Na5-c6 .. Ne14-f12 .. m8-l8 )  .. b7-c7 .. g13-g12 .. Nn10-l9"];
    for example in examples {
        let pgn: PGN4 = example.parse().unwrap();
        let string = pgn.to_string();
        assert_eq!(example, string, "Pgn not inverse");
    }
}
#[test]
fn long_boy() {
    let long = "1. h2-h3 .. b8-c8 .. g13-g12 .. m7-l7
2. Qg1xQn8+ .. Qa7xQh14+ .. Kg14xQh14 .. Kn7xQn8
3. Bi1xBa9 .. Ka8xBa9 .. Bf14xBn6 .. m4-l4
4. e2-e3 .. Na10-c9 .. Bn6-j10 .. Nn5-l6
5. Ne1-f3 .. Na5-c6 .. j13-j12 .. Kn8-n7
6. Nf3-e5 .. Nc6xNe5 .. Bj10xNe5+ .. Nn10-l9
7. Bf1xb5 .. Ka9-a8 .. Bi14-j13 .. Nl6-k4
8. Bb5xRa4 .. b11-d11 .. Be5-f6 .. m5-k5
9. Rd1-e1 .. d11-e11 .. Ne14-d12 .. m11-k11
10. Ba4-h11 .. e11-f11 .. g12xf11 .. k11xj12
11. Bh11-g10 .. Ra11-d11 .. k13xj12 .. Rn11-j11
12. j2-j3 .. Nc9-d7 .. Bf6xb10 .. Nk4-j6
13. d2-d4 .. Ba6-b5 .. Bb10-j2 .. Rj11-j8
14. Nj1-i3 .. Rd11-e11 .. e13-e12 .. l4-k4
15. Ni3xk4 .. Nd7-e9 .. Bj2-m5 .. Rn4xNk4
16. j3xRk4 .. Bb5xBg10 .. f11xBg10 .. Nj6xk4
17. Kh1-h2 .. Re11-g11 .. Bm5xRj8 .. Nl9xBj8
18. Rk1-j1 .. Ne9xg10 .. h13-h12 .. Nj8-k10
19. Rj1-j10 .. Ng10-i9 .. h12xRg11 .. m9-k9
20. Rj10-b10 .. Ka8-a9 .. Nd12-b11+ .. m8-k8
21. Rb10xNk10 .. Ka9-b8 .. Bj13-i12 .. Bn9-m8
22. Rk10xk9 .. b9-d9 .. Bi12xBm8+ .. Kn7xBm8
23. Rk9xNi9 .. b6-d6 .. Nj14-k12 .. l7-k7
24. Re1-j1 .. b7-d7 .. Nk12-l10+ .. Km8-l8
25. Rj1-j4 .. b4-c4 .. Rk14xk8+ .. Kl8-l7
26. Rj4xNk4 .. d6-e6 .. Rk8-n8 .. Kl7-k6
27. Rk4-m4 .. e6-f6 .. Nl10-n9 .. m6-l6
28. Ri9xd9 .. f6-g6 .. Nn9-m7+ .. Kk6-l7
29. Rd9xd7 .. g6-h6 .. Nb11-d10 .. m10-k10
30. Rd7-d6 .. h6-i6 .. Rn8xc8+ .. k5-j5
31. Rd6-b6+ .. Kb8-a7 .. Rc8-c11 .. k7-j7";

    let pgn: PGN4 = long.parse().unwrap();
    let string = pgn.to_string();
    assert_eq!(long, string, "Pgn not inverse");
}

#[test]
fn with_header() {
    let header = "[Variant \"Teams\"]
[RuleVariants \"EnPassant\"]
[CurrentMove \"0\"]
[StartFen4 \"R-0,0,0,0-1,1,1,1-1,1,1,1-0,0,0,0-0-3,yR,yN,yB,yK,yQ,yB,yN,yR,3/3,yP,yP,yP,yP,yP,yP,yP,yP,3/14/bR,11,gP,gR/bN,bP,10,gP,gN/bB,bP,10,gP,gB/bK,bP,10,gP,gQ/bQ,bδ,10,gP,gK/bB,bγ,10,gP,gB/bN,bβ,10,gP,gN/bR,bα,10,gP,gR/14/3,rP,rP,rP,rP,rP,rP,rP,rP,3/3,rR,rN,rB,rQ,rK,rB,rN,rR,3\"]

1. d2-d4 .. βb5-d5 .. g13-g12 .. m10-l10
2. d4xβc5";

    let pgn: PGN4 = header.parse().unwrap();
    let string = pgn.to_string();
    assert_eq!(header, string, "Pgn not inverse");
}
