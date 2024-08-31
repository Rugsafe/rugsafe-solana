import numpy as np
import matplotlib.pyplot as plt

# Define time and parameters for emission and burn rates
time = np.linspace(1, 100, 500)  # Start from 1 to avoid log(0)
emission_rate = 10
burn_rate = 2

# Calculate the emission and burn values separately
emission = emission_rate * time
burn = burn_rate * np.log(time)

# Plotting the emission component
plt.figure(figsize=(10, 6))
plt.plot(time, emission, color='cyan', label='Emission')
plt.xlabel('Time', color='white')
plt.ylabel('Emission', color='white')
plt.title('Rugsafe Token Emission Over Time', color='white')
plt.grid(True, color='gray')
plt.legend()
plt.tick_params(axis='x', colors='white')
plt.tick_params(axis='y', colors='white')
plt.legend(facecolor='black', edgecolor='white')
plt.show()

# Plotting the burn component
plt.figure(figsize=(10, 6))
plt.plot(time, burn, color='red', label='Burn')
plt.xlabel('Time', color='white')
plt.ylabel('Burn', color='white')
plt.title('Rugsafe Token Burn Over Time', color='white')
plt.grid(True, color='gray')
plt.legend()
plt.tick_params(axis='x', colors='white')
plt.tick_params(axis='y', colors='white')
plt.legend(facecolor='black', edgecolor='white')
plt.show()

