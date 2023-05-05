import matplotlib.pyplot as plt
import numpy as np

# Load data from CSV file
data = np.genfromtxt('1000_games.dat', delimiter=',')
plt.rcParams['text.usetex'] = True

# Plot histograms for each player
fig, ax = plt.subplots()
colors = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728']
for i in range(4):
    ax.hist(data[:, i], alpha=0.7, label=f'Player {i+1}', density=True, bins=150, color=colors[i])

# Set axis labels
ax.set_xlabel(r'Points at the end of a Game')
ax.set_ylabel(r'Density')

ax.legend()
ax.grid()
fig.savefig("point_distribution_1000.png", dpi = 200, bbox_inches='tight')
plt.close(fig)