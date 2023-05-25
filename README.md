# Mahjong Simulator

This program is used to test different mahjong strategies against each other and see how well they perform. 

Requirements:
  * rust
  * python
  * latex
  * others(?)

### Basic usage:
Create game data by running the core program. This simulates a lot of games and tallies up the points into `1000_games.dat`
```
cargo run --release
```
We can then plot our data into graphs to easily view how the strategies performed.
```
python distribution_plotter.py
python cumulative_plotter.py
```
These graphs are generated in the project root directory with names `point_distribution_1000.png` and `game_plot` which you can admire with your favorite image viewer.
