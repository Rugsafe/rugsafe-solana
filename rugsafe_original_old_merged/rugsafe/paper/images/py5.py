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





holdings_Ca = np.linspace(1, 1000, 500)
lambda_values = [1.5, 2.0, 2.5]
for lambd in lambda_values:
    penalty = holdings_Ca**lambd
    plt.plot(holdings_Ca, penalty, label=f'λ = {lambd}')

plt.xlabel('Holdings of $C_a$', color='white')
plt.ylabel('Penalty $P(C_a)$', color='white')
plt.title('Whale Penalty Impact Based on Holdings of $C_a$', color='white')
plt.grid(True, color='gray')
plt.legend()
plt.tick_params(axis='x', colors='white')
plt.tick_params(axis='y', colors='white')
plt.legend(facecolor='black', edgecolor='white')
plt.show()





expected_value_Cr = np.linspace(1, 100, 500)
market_value_Ca = expected_value_Cr**1.1
plt.figure(figsize=(10, 6))
plt.plot(expected_value_Cr, market_value_Ca, color='green', label=r'Market Potential of $C_a$ Tokens')
plt.xlabel('Expected Value of Rugged Tokens $E(P_{C_r})$', color='white')
plt.ylabel('Market Value of $C_a$', color='white')
plt.title('Market Potential for $C_a$ Tokens', color='white')
plt.grid(True, color='gray')
plt.legend()
plt.tick_params(axis='x', colors='white')
plt.tick_params(axis='y', colors='white')
plt.legend(facecolor='black', edgecolor='white')
plt.show()





time = np.linspace(0, 100, 500)
emission_rate = 10
burn_rate = 2
R_net = emission_rate * time - burn_rate * np.log(time + 1)
plt.figure(figsize=(10, 6))
plt.plot(time, R_net, color='magenta', label=r'Net Supply of Rugsafe Tokens $R_\text{net}$')
plt.xlabel('Time', color='white')
plt.ylabel('Net Supply of Rugsafe Tokens $R_\text{net}$', color='white')
plt.title('Emission and Burn Dynamics of Rugsafe Tokens', color='white')
plt.grid(True, color='gray')
plt.legend()
plt.tick_params(axis='x', colors='white')
plt.tick_params(axis='y', colors='white')
plt.legend(facecolor='black', edgecolor='white')
plt.show()








utility_Ca = np.linspace(1, 100, 500)
liquidity_Ca = utility_Ca**0.8
plt.figure(figsize=(10, 6))
plt.plot(utility_Ca, liquidity_Ca, color='orange', label=r'Liquidity of $C_a$ Tokens')
plt.xlabel('Utility of $C_a$', color='white')
plt.ylabel('Liquidity of $C_a$', color='white')
plt.title('Utility and Liquidity of $C_a$ Tokens', color='white')
plt.grid(True, color='gray')
plt.legend()
plt.tick_params(axis='x', colors='white')
plt.tick_params(axis='y', colors='white')
plt.legend(facecolor='black', edgecolor='white')
plt.show()
