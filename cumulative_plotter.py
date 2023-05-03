import matplotlib.pyplot as plt
import numpy as np

data = np.genfromtxt('10000_games.dat', delimiter=',')
N_games = len(data)
player1 = np.cumsum(data[:, 0] - 25000)
player2 = np.cumsum(data[:, 1] - 25000)
player3 = np.cumsum(data[:, 2] - 25000)
player4 = np.cumsum(data[:, 3] - 25000)

plt.rcParams['text.usetex'] = True

fig, ax = plt.subplots()

ax.plot(player1, label='Player 1')
ax.plot(player2, label='Player 2')
ax.plot(player3, label='Player 3')
ax.plot(player4, label='Player 4')

ax.set_xlabel(r'Games')
ax.set_ylabel(r'Cumulative Point Difference')
ax.set_xlim(0, 10000)
ax.legend()
ax.grid()
fig.savefig("game_plot.png", dpi = 200, bbox_inches='tight')
plt.close(fig)
