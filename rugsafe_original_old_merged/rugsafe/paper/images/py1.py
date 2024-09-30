import numpy as np
import matplotlib.pyplot as plt

# Define the range of the sum of rugged token values
rugged_token_values = np.linspace(1, 100, 500)

# Calculate the Rugsafe token price based on the equation
rugsafe_price = 1 / np.log(rugged_token_values)
rugsafe_price[0] = 0  # Avoid infinite value at the start

# Calculate the price after burning
burn_percentage = 0.5  # Example of burning 50% of the tokens
burned_rugsafe_price = 1 / np.log(rugged_token_values * (1 - burn_percentage))
burned_rugsafe_price[0] = 0  # Avoid infinite value at the start

# Set up the plot with a black background and white text
plt.style.use('dark_background')

# Plot the results
fig, axs = plt.subplots(1, 2, figsize=(14, 6))

# Plot for minting
axs[0].plot(rugged_token_values, rugsafe_price, color='orange', label='Rugsafe Token Price')
axs[0].set_xlabel('Sum of Rugged Token Values', color='white')
axs[0].set_ylabel('Rugsafe Token Price', color='white')
axs[0].set_title('Rugsafe Token Price vs. Sum of Rugged Token Values', color='white')
axs[0].grid(True, color='gray')
axs[0].legend()

# Plot for burning
axs[1].plot(rugged_token_values, burned_rugsafe_price, color='red', label='Burned Rugsafe Token Price')
axs[1].set_xlabel('Sum of Rugged Token Values', color='white')
axs[1].set_ylabel('Burned Rugsafe Token Price', color='white')
axs[1].set_title('Burned Rugsafe Token Price vs. Sum of Rugged Token Values', color='white')
axs[1].grid(True, color='gray')
axs[1].legend()

# Adjusting the colors of the ticks and labels
for ax in axs:
    ax.tick_params(axis='x', colors='white')
    ax.tick_params(axis='y', colors='white')
    ax.legend(facecolor='black', edgecolor='white')

# Save the plot as an image file
plt.savefig('rugsafe_price_vs_burned_black_background.png')

# Display the plot
plt.show()

