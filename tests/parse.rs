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
31. Rd6-b6+ .. Kb8-a7 .. Rc8-c11 .. k7-j7
32. O-O#";

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

#[test]
fn with_long_header() {
    let long_header = "[GameNr \"4177881\"]
[TimeControl \"3+0\"]
[Variant \"Teams\"]
[RuleVariants \"EnPassant Prom=8 PromoteTo=VHE\"]
[StartFen4 \"R-0,0,0,0-0,0,0,0-0,0,0,0-0,0,0,0-2-{'pawnsBaseRank':3}-
3,X,X,X,X,X,X,X,X,3/
3,X,yG,yY,yM,yK,yY,yG,X,3/
3,X,yδ,yδ,yδ,yδ,yδ,yδ,X,3/
X,X,X,X,6,X,X,X,X/
X,bG,bδ,8,gδ,gG,X/
X,bY,bδ,8,gδ,gY,X/
X,bK,bδ,8,gδ,gM,X/
X,bM,bδ,8,gδ,gK,X/
X,bY,bδ,8,gδ,gY,X/
X,bG,bδ,8,gδ,gG,X/
X,X,X,X,6,X,X,X,X/
3,X,rδ,rδ,rδ,rδ,rδ,rδ,X,3/
3,X,rG,rY,rK,rM,rY,rG,X,3/
3,X,X,X,X,X,X,X,X,3\"]
[Red \"Nthniel_Mullodzhanov\"]
[RedElo \"1570\"]
[Blue \"EyeoftheTiger1204\"]
[BlueElo \"2243\"]
[Yellow \"AGR1815\"]
[YellowElo \"1540\"]
[Green \"6_K\"]
[GreenElo \"2062\"]
[Result \"1-0\"]
[Termination \"Checkmate. 1-0\"]
[Site \"www.chess.com/4-player-chess\"]
[Date \"Fri Jul 17 2020 17:23:11 GMT+0000 (Coordinated Universal Time)\"]



1. δi3-i4 { date=2020-07-17T17:13:12.297Z clock=178312 }  .. δc9-e9 { date=2020-07-17T17:13:21.773Z clock=170812 }  .. δh12-h10 { date=2020-07-17T17:13:27.087Z clock=174887 }  .. Gm10-k8 { date=2020-07-17T17:13:39.875Z clock=167423 } 
2. δe3-e4 { date=2020-07-17T17:13:48.695Z clock=169522 }  .. Mb7-d6 { date=2020-07-17T17:13:56.786Z clock=162978 }  .. Yf13-h11 { date=2020-07-17T17:14:04.957Z clock=166924 }  .. Gk8xGe2+ { date=2020-07-17T17:14:07.665Z clock=164925 } 
3. Yf2-f4 { date=2020-07-17T17:14:17.832Z clock=159386 }  .. Md6-e6 { date=2020-07-17T17:14:23.256Z clock=157788 }  .. Ge13-g11 { date=2020-07-17T17:14:33.673Z clock=156714 }  .. Ym6-k6 { date=2020-07-17T17:14:39.635Z clock=159174 } 
4. Mh2-i3 { date=2020-07-17T17:14:55.585Z clock=143468 }  .. Me6xYf4+ { date=2020-07-17T17:14:58.681Z clock=154938 }  .. δf12-f11 { date=2020-07-17T17:15:03.826Z clock=151775 }  .. Yk6xδi4+ { date=2020-07-17T17:15:05.285Z clock=157927 } 
5. Kg2-f2 { date=2020-07-17T17:15:13.297Z clock=135524 }  .. Mf4-g6 { date=2020-07-17T17:15:26.775Z clock=141711 }  .. Mg13-f12 { date=2020-07-17T17:15:28.515Z clock=150237 }  .. Yi4xYi2 { date=2020-07-17T17:15:30.678Z clock=155978 } 
6. Kf2xGe2 { date=2020-07-17T17:15:36.916Z clock=129352 }  .. Gb10xGj2 { date=2020-07-17T17:15:41.553Z clock=137336 }  .. Mf12-e10 { date=2020-07-17T17:15:43.050Z clock=148943 }  .. δl7-j7 { date=2020-07-17T17:16:01.020Z clock=138222 } 
7. Mi3xGj2 { date=2020-07-17T17:16:10.797Z clock=119639 }  .. δc10-d9 { date=2020-07-17T17:16:13.129Z clock=135263 }  .. Gg11-g5 { date=2020-07-17T17:16:15.897Z clock=146370 }  .. δj7-i7 { date=2020-07-17T17:16:26.200Z clock=128137 } 
8. Mj2xYi2 { date=2020-07-17T17:16:27.969Z clock=117934 }  .. δd9xMe10 { date=2020-07-17T17:16:32.272Z clock=131217 }  .. Gg5xGb5 { date=2020-07-17T17:16:34.763Z clock=144075 }  .. δi7-h7 { date=2020-07-17T17:16:36.901Z clock=126218 } 
9. δh3-h5 { date=2020-07-17T17:16:45.221Z clock=109675 }  .. δe10xδf11 { date=2020-07-17T17:16:48.114Z clock=128567 }  .. δg12xδf11 { date=2020-07-17T17:16:52.906Z clock=139481 }  .. δh7-g8=E { date=2020-07-17T17:16:58.239Z clock=121105 } 
10. δh5xMg6 { date=2020-07-17T17:17:00.936Z clock=107006 }  .. δe9-f8 { date=2020-07-17T17:17:08.759Z clock=120998 }  .. δh10-h9 { date=2020-07-17T17:17:10.561Z clock=137877 }  .. Eg8xδg6 { date=2020-07-17T17:17:12.212Z clock=119674 } 
11. δf3-h5 { date=2020-07-17T17:17:19.590Z clock=99657 }  .. δf8-g9 { date=2020-07-17T17:17:21.173Z clock=119653 }  .. δh9-i8 { date=2020-07-17T17:17:23.928Z clock=135319 }  .. Eg6-f8 { date=2020-07-17T17:17:34.533Z clock=109292 } 
12. δh5-h6 { date=2020-07-17T17:17:38.474Z clock=95746 }  .. δg9-h10=V { date=2020-07-17T17:17:56.624Z clock=101740 }  .. δi8-h7=E+ { date=2020-07-17T17:18:06.773Z clock=125368 }  .. Ef8xEh7 { date=2020-07-17T17:18:08.945Z clock=107339 } 
13. δh6xEh7 { date=2020-07-17T17:18:11.472Z clock=93248 }  .. Vh10-e9 { date=2020-07-17T17:18:23.325Z clock=90148 }  .. δf11-f10 { date=2020-07-17T17:18:26.353Z clock=122539 }  .. Mm8-k7 { date=2020-07-17T17:18:29.461Z clock=104449 } 
14. Mi2-h4 { date=2020-07-17T17:18:40.493Z clock=82247 }  .. Ve9-f12+ { date=2020-07-17T17:18:48.769Z clock=82137 }  .. Kh13-g12 { date=2020-07-17T17:18:55.320Z clock=116185 }  .. Mk7-j9 { date=2020-07-17T17:18:58.618Z clock=101368 } 
15. δh7-g8=V { date=2020-07-17T17:19:03.427Z clock=77466 }  .. Vf12-g10 { date=2020-07-17T17:19:05.852Z clock=79984 }  .. Yh11xMj9 { date=2020-07-17T17:19:08.432Z clock=113838 }  .. δl8-k7 { date=2020-07-17T17:19:15.078Z clock=94940 } 
16. Mh4-j5 { date=2020-07-17T17:19:21.907Z clock=70663 }  .. Vg10-h8 { date=2020-07-17T17:19:37.411Z clock=64785 }  .. Yj9-h7 { date=2020-07-17T17:19:46.928Z clock=104555 }  .. δl5-k6 { date=2020-07-17T17:19:50.047Z clock=92039 } 
17. Mj5xδk6+ { date=2020-07-17T17:19:59.617Z clock=61121 }  .. δc6-e6 { date=2020-07-17T17:20:10.535Z clock=54179 }  .. Yh7-j5 { date=2020-07-17T17:20:18.251Z clock=97073 }  .. δl6xMk6 { date=2020-07-17T17:20:18.474Z clock=92009 } 
18. δg3-h4 { date=2020-07-17T17:20:35.387Z clock=44236 }  .. δe6-f7 { date=2020-07-17T17:20:37.666Z clock=52239 }  .. Yj5-h7 { date=2020-07-17T17:20:41.553Z clock=93420 }  .. Gm5xGb5 { date=2020-07-17T17:20:45.541Z clock=88238 } 
19. Vg8-f11 { date=2020-07-17T17:20:51.881Z clock=37923 }  .. δf7-g7 { date=2020-07-17T17:20:54.559Z clock=49887 }  .. δi12-i11 { date=2020-07-17T17:21:11.624Z clock=76589 }  .. δk7-j7 { date=2020-07-17T17:21:18.640Z clock=81442 } 
20. Vf11-c10+ { date=2020-07-17T17:21:25.749Z clock=30842 }  .. Kb8-c9 { date=2020-07-17T17:21:31.847Z clock=44089 }  .. δf10-f9 { date=2020-07-17T17:21:32.962Z clock=75676 }  .. δj7-i7 { date=2020-07-17T17:21:36.267Z clock=78358 } 
21. Vc10-b7+ { date=2020-07-17T17:21:38.455Z clock=28683 }  .. Yb9xVb7 { date=2020-07-17T17:21:42.601Z clock=40198 }  .. Yh7-h5 { date=2020-07-17T17:21:50.811Z clock=67668 }  .. δk6-j6 { date=2020-07-17T17:21:55.968Z clock=73420 } 
22. δh4-g5 { date=2020-07-17T17:21:58.030Z clock=26650 }  .. Vh8xδg5 { date=2020-07-17T17:22:01.848Z clock=36648 }  .. δf9-f8 { date=2020-07-17T17:22:03.122Z clock=66595 }  .. δi7-h8 { date=2020-07-17T17:22:04.342Z clock=72419 } 
23. δe4-e5 { date=2020-07-17T17:22:09.146Z clock=21877 }  .. δg7-h6=E { date=2020-07-17T17:22:15.754Z clock=30277 }  .. δf8-e7=E+ { date=2020-07-17T17:22:19.294Z clock=63256 }  .. δj6-i7 { date=2020-07-17T17:22:24.160Z clock=67778 } 
24. δe5-f6 { date=2020-07-17T17:22:27.846Z clock=18223 }  .. Eh6xYh5 { date=2020-07-17T17:22:30.543Z clock=27831 }  .. Ee7-e9+ { date=2020-07-17T17:22:40.688Z clock=53313 }  .. δi7-h7 { date=2020-07-17T17:22:47.908Z clock=60779 } 
25. δf6-e7 { date=2020-07-17T17:22:50.828Z clock=15335 }  .. Kc9-b8 { date=2020-07-17T17:22:53.197Z clock=25722 }  .. Ee9-c10+ { date=2020-07-17T17:23:02.592Z clock=44116 }  .. δh7-g7=E+ { date=2020-07-17T17:23:04.937Z clock=58653 } 
26. δe7-d8=V { date=2020-07-17T17:23:11.201Z clock=9101 }  .. # { date=2020-07-17T17:23:11.201Z clock=25722 } ";

    let pgn: PGN4 = long_header.parse().unwrap();
    let string = pgn.to_string();
    for (a, b) in long_header.lines().zip(string.lines()) {
        if a != b {
            println!("{}\n{}", a, b);
            let a_w: Vec<&str> = a.split_whitespace().collect();
            let b_w: Vec<&str> = b.split_whitespace().collect();
            if a_w != b_w {
                assert_eq!(a, b, "Pgn not inverse");
            }
        }
    }
}
