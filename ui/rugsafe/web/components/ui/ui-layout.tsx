'use client';

import * as React from 'react';
import { ReactNode, Suspense, useEffect, useRef } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { WalletButton } from '../solana/solana-provider';
import { AccountChecker } from '../account/account-ui';
import { ClusterChecker, ClusterUiSelect, ExplorerLink } from '../cluster/cluster-ui';
import toast, { Toaster } from 'react-hot-toast';
import CreateVault from '../solana/CreateVault';

export function UiLayout({
  children,
  links,
}: {
  children: ReactNode;
  links: { label: string; path: string }[];
}) {
  const pathname = usePathname();

  return (
    <div className="h-full w-full flex flex-col bg-[#0a0b1e] text-white">
      <div className="navbar bg-base-300 text-neutral-content flex-col md:flex-row space-y-2 md:space-y-0 w-full">
        <div className="flex-1">
          <Link className="btn btn-ghost normal-case text-xl" href="/">
            <img className="h-8 md:h-8" alt="Logo" src="assets/img/rugsafe.png" />
          </Link>
          <ul className="menu menu-horizontal px-1 space-x-2">
            {links.map(({ label, path }) => (
              <li key={path}>
                <Link
                  className={pathname.startsWith(path) ? 'active' : ''}
                  href={path}
                >
                  {label}
                </Link>
              </li>
            ))}
          </ul>
        </div>
        <div className="flex-none space-x-2">
          <WalletButton />
          <ClusterUiSelect />
        </div>
      </div>

      <ClusterChecker>
        <AccountChecker />
      </ClusterChecker>

      <div className="flex-grow w-full">
        <Suspense
          fallback={
            <div className="text-center my-32">
              <span className="loading loading-spinner loading-lg"></span>
            </div>
          }
        >
          {children}
        </Suspense>
        <Toaster position="bottom-right" />
      </div>

      <footer className="footer footer-center p-4 bg-base-300 text-base-content w-full">
        <aside>
          <p>Rugsafe Foundation</p>
        </aside>
      </footer>
    </div>
  );
}



export function AppHero({
  children,
  title,
  subtitle,
}: {
  children?: React.ReactNode;
  title: React.ReactNode;
  subtitle: React.ReactNode;
}) {
  const data = [
    { name: 'Jan', value: 10 },
    { name: 'Feb', value: 34 },
    { name: 'Mar', value: 89 },
    { name: 'Apr', value: 199 },
    { name: 'May', value: 149 },
    { name: 'Jun', value: 246 },
  ];

  return (
    <div className="grid grid-cols-1 md:grid-cols-4 gap-8 mb-12">
      <div className="md:col-span-4 bg-gradient-to-r from-purple-600 to-pink-500 rounded-3xl shadow-lg p-8 mb-8">
        <h1 className="text-5xl font-bold mb-4">{title}</h1>
        <p className="text-xl">{subtitle}</p>
      </div>
      <div className="md:col-span-1 bg-[#1a1b2e] rounded-3xl shadow-lg p-6">
        <h2 className="text-3xl font-bold mb-2">$1,456</h2>
        <p className="text-gray-300 mb-4">Total Value Locked</p>
        <div className="h-40">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={data}>
              <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.1)" />
              <XAxis dataKey="name" stroke="#fff" fontSize={12} tickLine={false} axisLine={false} />
              <YAxis hide />
              <Tooltip 
                contentStyle={{ backgroundColor: '#1a1b2e', border: 'none', borderRadius: '0.5rem' }}
                itemStyle={{ color: '#fff' }}
              />
              <Line type="monotone" dataKey="value" stroke="#8884d8" strokeWidth={2} dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>
      <div className="md:col-span-3 space-y-6">
        <div className="bg-[#1a1b2e] rounded-3xl shadow-lg p-6">
          <CreateVault />
        </div>
        <div className="bg-[#1a1b2e] rounded-3xl shadow-lg p-6">
          <h2 className="text-2xl font-bold mb-4">Filters</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <input
                type="text"
                placeholder="Search YFI Vault"
                className="w-full px-4 py-2 bg-[#2a2b3e] rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500"
              />
            </div>
            <div>
              <select className="w-full px-4 py-2 bg-[#2a2b3e] rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-purple-500">
                <option>Select Blockchain</option>
                <option>Ethereum</option>
                <option>Solana</option>
                <option>Binance Smart Chain</option>
              </select>
            </div>
            <div>
              <select className="w-full px-4 py-2 bg-[#2a2b3e] rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-purple-500">
                <option>Select Type</option>
                <option>Fancy</option>
                <option>Standard</option>
              </select>
            </div>
          </div>
        </div>
      </div>
      <div className="md:col-span-4 bg-gradient-to-r from-pink-500 to-purple-600 rounded-3xl shadow-lg p-6">
      <h2 className="text-2xl font-bold mb-2">"Oh my vault... look at that APY!"</h2>
<p className="text-lg">
  In Rugsafe, V3 Vaults are enhanced with auto-compounding rewards and fortified security. Sit back, relax, and trust that your assets are safe while enjoying those secure returns.
</p>

      </div>
    </div>
  );
}

export function AppModal({
  children,
  title,
  hide,
  show,
  submit,
  submitDisabled,
  submitLabel,
}: {
  children: ReactNode;
  title: string;
  hide: () => void;
  show: boolean;
  submit?: () => void;
  submitDisabled?: boolean;
  submitLabel?: string;
}) {
  const dialogRef = useRef<HTMLDialogElement | null>(null);

  useEffect(() => {
    if (!dialogRef.current) return;
    if (show) {
      dialogRef.current.showModal();
    } else {
      dialogRef.current.close();
    }
  }, [show, dialogRef]);

  return (
    <dialog className="modal" ref={dialogRef}>
      <div className="modal-box space-y-5">
        <h3 className="font-bold text-lg">{title}</h3>
        {children}
        <div className="modal-action">
          <div className="join space-x-2">
            {submit ? (
              <button
                className="btn btn-xs lg:btn-md btn-primary"
                onClick={submit}
                disabled={submitDisabled}
              >
                {submitLabel || 'Save'}
              </button>
            ) : null}
            <button onClick={hide} className="btn">
              Close
            </button>
          </div>
        </div>
      </div>
    </dialog>
  );
}

export function ellipsify(str = '', len = 4) {
  if (str.length > 30) {
    return (
      str.substring(0, len) + '..' + str.substring(str.length - len, str.length)
    );
  }
  return str;
}

export function useTransactionToast() {
  return (signature: string) => {
    toast.success(
      <div className={'text-center'}>
        <div className="text-lg">Transaction sent</div>
        <ExplorerLink
          path={`tx/${signature}`}
          label={'View Transaction'}
          className="btn btn-xs btn-primary"
        />
      </div>
    );
  };
}
