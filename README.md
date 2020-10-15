pgn4
====

A rust library that parses PGN4 files.

PGN4 is an extension of the [PGN format](https://en.wikipedia.org/wiki/Portable_Game_Notation) 
for 4 player chess. It is used by [Chess.com](https://www.chess.com/4-player-chess)
uses the format to store the 4 player chess games stored on the site. Because
the format accommodates many types of variants of 4 player chess the format has
a bunch on complexities to correctly parsing any valid game. 

The format parsed into is likely more complex than any tool will want to directly
handle, but it captures all of the information in the PGN4 file.
