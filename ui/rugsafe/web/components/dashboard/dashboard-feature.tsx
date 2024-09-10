'use client';

import { AppHero } from '../ui/ui-layout';
import CreateVault from '../solana/CreateVault'; // Import the CreateVault component
import NewListVaults from '../NewListVault';
import ListVaultsFromRegistry from '../ListVaultsFromRegistry';
import NewVaultList from '../dummylist';
import ListVaults from '../ListVault';



const links: { label: string; href: string }[] = [
  { label: 'Solana Docs', href: 'https://docs.solana.com/' },
  { label: 'Solana Faucet', href: 'https://faucet.solana.com/' },
  { label: 'Solana Cookbook', href: 'https://solanacookbook.com/' },
  { label: 'Solana Stack Overflow', href: 'https://solana.stackexchange.com/' },
  {
    label: 'Solana Developers GitHub',
    href: 'https://github.com/solana-developers/',
  },
];

export default function DashboardFeature() {
  return (
    <div>
      <AppHero title="RugSafe" subtitle="Deposit Rugged Tokens into Vault" />
      {/* <div className="max-w-xl mx-auto py-6 sm:px-6 lg:px-8 text-center">
        Create a vault, or faucet tokens to test
      </div> */}
      {/* <CreateVault /> */}
      {/* <ListVaults /> */}
      <ListVaultsFromRegistry />
    </div>
  );
}
