# Mahjong Simulator

Some ideas for general structure:

Tile (Hai) struct:
  + [enum] Suit (m, p, s, k, s)
  + [const int] value (1 - 9) for m,p,s (1 - 4) for k and (1 - 3) for s
  + [mut bool] dora

Boardstate:
  + [mut List[Hai]] Wall
  + [mut List[Hai]] Deadwall
  + [mut List[Hai]] DoraIndicator
  + [mut int] Honba
  + [mut int] Riichi sticks
  + [mut int] Round Wind

Player struct:
  + [mut int] Points
  + [mut int] Seat wind
  + [mut List[Hai]] Hand
  + [mut List[Hai]] Discards
  + [const List[Strategy]] DiscardStrategies
  + [const List[Strategy]] CallingStrategies
  + [mut bool] Riichistate
  + [mut bool] OpenHand
