import matplotlib.pyplot as plt
import numpy as np

# Price range for the rugged token
rugged_token_price_up = np.linspace(0.1, 10, 100)    # Price going up
rugged_token_price_down = np.linspace(10, 0.1, 100)  # Price going down

# Anticoin price calculated as a logarithmic inverse of rugged token price
anticoin_price_up = np.log(10 / rugged_token_price_up)
anticoin_price_down = np.log(rugged_token_price_up)  # Adjusted to taper off as Cr decreases

# Adjust anticoin prices to match the initial value with rugged token price
anticoin_price_up = anticoin_price_up + (rugged_token_price_up[0] - anticoin_price_up[0])
anticoin_price_down = anticoin_price_down + (rugged_token_price_down[0] - anticoin_price_down[0])

# Create a figure with two subplots side by side, set background to black and labels to white
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 6), facecolor='black')

# First subplot: Cr price going up and Ca price going down
ax1.plot(rugged_token_price_up, rugged_token_price_up, color='purple', label="Rugged Token Price ($C_r$)")
ax1.plot(rugged_token_price_up, anticoin_price_up, color='blue', label="Anticoin Price ($C_a$)")
ax1.set_title("$C_r$ Up, $C_a$ Log Down", color='white')
ax1.set_xlabel("Price of Rugged Token ($C_r$)", color='white')
ax1.set_ylabel("Price", color='white')
ax1.legend(facecolor='black', edgecolor='white', labelcolor='white')
ax1.set_facecolor('black')
ax1.tick_params(axis='x', colors='white')
ax1.tick_params(axis='y', colors='white')
ax1.grid(True, color='gray', linestyle='--', linewidth=0.5)  # Less prominent grid lines

# Second subplot: Cr price going down and Ca price going up (now tapering off correctly)
ax2.plot(rugged_token_price_up, rugged_token_price_down, color='purple', label="Rugged Token Price ($C_r$)")
ax2.plot(rugged_token_price_up, anticoin_price_down, color='blue', label="Anticoin Price ($C_a$)")
ax2.set_title("$C_r$ Down, $C_a$ Log Up", color='white')
ax2.set_xlabel("Price of Rugged Token ($C_r$)", color='white')
ax2.set_ylabel("Price", color='white')
ax2.legend(facecolor='black', edgecolor='white', labelcolor='white')
ax2.set_facecolor('black')
ax2.tick_params(axis='x', colors='white')
ax2.tick_params(axis='y', colors='white')
ax2.grid(True, color='gray', linestyle='--', linewidth=0.5)  # Less prominent grid lines

# Set the entire figure background to black
plt.tight_layout()
plt.show()

