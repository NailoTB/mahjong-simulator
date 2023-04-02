# Mahjong Simulator

Some ideas for general structure:

Tile (Hai) struct:
  + [const int or string] Suit (m, p, s, z)
  + [const int] value (1 - 9) for m,p,s and (1 - 7) for z (eswn, haku hatu chuu)
  + [mut bool] dora

Boardstate:
  + [mut List[Hai]] Wall
  + [mut List[Hai]] Deadwall
  + [mut List[Hai]] DoraIndicator
  + [mut int] Honba
  + [mut int] Riichi sticks

Player struct:
  + [mut int] Points
  + [mut int] Seat wind
  + [mut List[Hai]] Discards
  + [mut List[Hai]] Strategies
  + [mut bool] Riichistate
