import React from 'react';
import { ChevronDown } from 'lucide-react';
import Link from 'next/link';

const vaults = [
  {
    name: 'DAI',
    description: 'Dai Stablecoin',
    icon: '/placeholder.svg?height=40&width=40',
    chain: 'Ethereum',
    estApy: { current: 5.57, boost: 6.94 },
    histApy: 6.01,
    available: 0.00,
    holdings: 0.00,
    deposits: 3.91,
  },
  {
    name: 'USDT',
    description: 'Tether USD',
    icon: '/placeholder.svg?height=40&width=40',
    chain: 'Ethereum',
    estApy: { current: 3.33, boost: null },
    histApy: 3.31,
    available: 0.00,
    holdings: 0.00,
    deposits: 6.33,
  },
  {
    name: 'yPT-USDe',
    description: 'Yearn Auto-Rolling Pendle PT',
    icon: '/placeholder.svg?height=40&width=40',
    chain: 'Arbitrum',
    estApy: { current: 10.10, boost: null },
    histApy: 7.60,
    available: 0.00,
    holdings: 0.00,
    deposits: 1.65,
    additionalInfo: '+ 2500 ARB per week 🚀',
  },
  {
    name: 'USDC',
    description: 'USD Coin',
    icon: '/placeholder.svg?height=40&width=40',
    chain: 'Ethereum',
    estApy: { current: 3.77, boost: null },
    histApy: 4.75,
    available: 0.00,
    holdings: 0.00,
    deposits: 2.61,
  },
];

const VaultItem = ({ vault }) => (
  <Link href="/components/NewListVault" className="block">
    <div className="bg-gradient-to-r from-purple-900 to-blue-900 rounded-lg p-4 hover:from-purple-600 hover:to-pink-600 transition-all duration-300 cursor-pointer">
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <img src={vault.icon} alt={vault.name} className="w-10 h-10 rounded-full" />
          <div>
            <h3 className="text-xl font-bold text-white">{vault.name}</h3>
            <p className="text-sm text-gray-300">{vault.description}</p>
            <span className="inline-block bg-blue-600 text-xs text-white px-2 py-1 rounded-full mt-1">{vault.chain}</span>
          </div>
        </div>
        <div className="flex items-center space-x-8 text-right">
          <div>
            <p className="text-lg font-bold text-white flex items-center">
              {vault.estApy.current.toFixed(2)}%
              {vault.estApy.boost && (
                <span className="text-yellow-400 ml-1">→ {vault.estApy.boost.toFixed(2)}%</span>
              )}
            </p>
            {vault.additionalInfo && <p className="text-xs text-yellow-400">{vault.additionalInfo}</p>}
            <p className="text-sm text-gray-400">Est. APY</p>
          </div>
          <div>
            <p className="text-lg font-bold text-white">{vault.histApy.toFixed(2)}%</p>
            <p className="text-sm text-gray-400">Hist. APY</p>
          </div>
          <div>
            <p className="text-lg font-bold text-gray-400">{vault.available.toFixed(2)}</p>
            <p className="text-sm text-gray-400">Available</p>
          </div>
          <div>
            <p className="text-lg font-bold text-gray-400">{vault.holdings.toFixed(2)}</p>
            <p className="text-sm text-gray-400">Holdings</p>
          </div>
          <div>
            <p className="text-lg font-bold text-white">{vault.deposits.toFixed(2)}M</p>
            <p className="text-sm text-gray-400">US${vault.deposits.toFixed(2)}M</p>
          </div>
        </div>
      </div>
    </div>
  </Link>
);

const NewVaultList = () => {
  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 bg-gray-900">
      <div className="flex justify-between items-center mb-6 text-gray-400">
        <div className="flex items-center space-x-4">
          <span className="flex items-center">Vault <ChevronDown className="ml-1 h-4 w-4" /></span>
          <span className="flex items-center">Est. APY <ChevronDown className="ml-1 h-4 w-4" /></span>
          <span className="flex items-center">Hist. APY <ChevronDown className="ml-1 h-4 w-4" /></span>
          <span className="flex items-center">Available <ChevronDown className="ml-1 h-4 w-4" /></span>
          <span className="flex items-center">Holdings <ChevronDown className="ml-1 h-4 w-4" /></span>
          <span className="flex items-center">Deposits <ChevronDown className="ml-1 h-4 w-4" /></span>
        </div>
      </div>
      <div className="space-y-2">
        {vaults.map((vault, index) => (
          <VaultItem key={index} vault={vault} />
        ))}
      </div>
    </div>
  );
};

export default NewVaultList;