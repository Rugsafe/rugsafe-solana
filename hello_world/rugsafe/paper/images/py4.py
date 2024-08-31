import numpy as np
import matplotlib.pyplot as plt

# Define a common style for all plots
plt.style.use('dark_background')

# Plot 1: Supply Regulation Plot
rugged_token_values = np.linspace(1, 1000, 500)
rugsafe_price = 1 / np.log(rugged_token_values)
rugsafe_supply = 1 / rugsafe_price

plt.figure(figsize=(10, 6))
plt.plot(rugged_token_values, rugsafe_supply, color='cyan', label=r'Theoretical Supply of Rugsafe Tokens $R$')
plt.xlabel(r'Sum of Rugged Token Values $\sum R_{C_v}$', color='white')
plt.ylabel('Theoretical Supply of Rugsafe Tokens $R$', color='white')
plt.title('Theoretical Supply of Rugsafe Tokens vs. Total Rugged Tokens', color='white')
plt.grid(True, color='gray')
plt.legend()
plt.tick_params(axis='x', colors='white')
plt.tick_params(axis='y', colors='white')
plt.legend(facecolor='black', edgecolor='white')
plt.show()

