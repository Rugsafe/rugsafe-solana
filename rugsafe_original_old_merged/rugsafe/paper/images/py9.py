import matplotlib.pyplot as plt
import numpy as np

# Price range for the rugged token
rugged_token_price_up = np.linspace(0.1, 10, 100)    # Price going up
rugged_token_price_down = np.linspace(10, 0.1, 100)  # Price going down

# Anticoin price calculated as a logarithmic inverse of rugged token price
anticoin_price_up = np.log(10 / rugged_token_price_up)
anticoin_price_down = np.log(10 / rugged_token_price_down)

# Create a figure with two subplots side by side
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 6))

# First subplot: Cr price going up and Ca price going down
ax1.plot(rugged_token_price_up, rugged_token_price_up, color='purple', label="Rugged Token Price ($C_r$)")
ax1.plot(rugged_token_price_up, anticoin_price_up, color='blue', label="Anticoin Price ($C_a$)")
ax1.set_title("$C_r$ Up, $C_a$ Log Down")
ax1.set_xlabel("Price of Rugged Token ($C_r$)")
ax1.set_ylabel("Price")
ax1.grid(True)
ax1.legend()

# Second subplot: Cr price going down (REVERSED X-AXIS) and Ca price going up
ax2.plot(rugged_token_price_up, rugged_token_price_down, color='purple', label="Rugged Token Price ($C_r$)")  # Reversing the rugged token price curve
ax2.plot(rugged_token_price_up, anticoin_price_down, color='blue', label="Anticoin Price ($C_a$)")
ax2.set_title("$C_r$ Down, $C_a$ Log Up")
ax2.set_xlabel("Price of Rugged Token ($C_r$)")
ax2.set_ylabel("Price")
ax2.grid(True)
ax2.legend()

# Display the plots
plt.tight_layout()
plt.show()

