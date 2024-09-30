import numpy as np
import matplotlib.pyplot as plt

# Define the range of the sum of rugged token values (total rugged tokens in the world)
rugged_token_values = np.linspace(1, 1000, 500)

# Calculate the Rugsafe token price based on the equation
rugsafe_price = 1 / np.log(rugged_token_values)

# Ensure the first value doesn't approach infinity (avoiding division by log(0))
rugsafe_price[0] = 0

# Calculate the corresponding theoretical supply of Rugsafe tokens
# Assuming the supply is inversely proportional to the price
rugsafe_supply = 1 / rugsafe_price

# Set up the plot with a black background and white text
plt.style.use('dark_background')

# Plot the results
plt.figure(figsize=(10, 6))
plt.plot(rugged_token_values, rugsafe_supply, color='cyan', label=r'Theoretical Supply of Rugsafe Tokens $R$')
plt.xlabel(r'Sum of Rugged Token Values $\sum R_{C_v}$', color='white')
plt.ylabel('Theoretical Supply of Rugsafe Tokens $R$', color='white')
plt.title('Theoretical Supply of Rugsafe Tokens vs. Total Rugged Tokens', color='white')
plt.grid(True, color='gray')
plt.legend(facecolor='black', edgecolor='white')

# Adjusting the colors of the ticks and labels
plt.tick_params(axis='x', colors='white')
plt.tick_params(axis='y', colors='white')

# Display the plot
plt.show()

