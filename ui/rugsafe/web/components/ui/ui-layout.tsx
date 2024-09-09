'use client';

import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

import { WalletButton } from '../solana/solana-provider';
import * as React from 'react';
import { ReactNode, Suspense, useEffect, useRef } from 'react';

import Link from 'next/link';
import { usePathname } from 'next/navigation';

import { AccountChecker } from '../account/account-ui';
import {
  ClusterChecker,
  ClusterUiSelect,
  ExplorerLink,
} from '../cluster/cluster-ui';
import toast, { Toaster } from 'react-hot-toast';


import CreateVault from '../solana/CreateVault'; // Import the CreateVault component


export function UiLayout({
  children,
  links,
}: {
  children: ReactNode;
  links: { label: string; path: string }[];
}) {
  const pathname = usePathname();

  return (
    <div className="h-full w-full flex flex-col"> {/* Set the layout container to take the full width */}
      <div className="navbar bg-base-300 text-neutral-content flex-col md:flex-row space-y-2 md:space-y-0 w-full"> {/* Add w-full to navbar */}
        <div className="flex-1">
          <Link className="btn btn-ghost normal-case text-xl" href="/">
            <img className="h-8 md:h-8" alt="Logo" src="assets/img/rugsafe.png" />
            {/* RugSafe */}
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

      <div className="flex-grow w-full"> {/* Removed mx-4 lg:mx-auto and added w-full to stretch */}
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

      <footer className="footer footer-center p-4 bg-base-300 text-base-content w-full"> {/* Ensure footer is full width */}
        <aside>
          <p>Rugsafe Foundation</p>
        </aside>
      </footer>
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

export function AppHero({
  children,
  title,
  subtitle,
}: {
  children?: ReactNode;
  title: ReactNode;
  subtitle: ReactNode;
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

    <div className="hero min-h-0 py-10 flex justify-center items-center" style={{ border: "0px solid green" }}>
 
      {/* Blue bordered div stretching across the full width */}
      <div style={{ border: "0px solid blue" }} className="flex flex-col md:flex-row justify-start items-center w-full text-white" >
        
        {/* First column: Value Display and Chart (25% width) */}
        <div className="hero-content shadow-2xl rounded-lg p-6 bg-opacity-80 md:w-1/4 gradient-bg flex flex-col items-center"
          style={{ backgroundSize: 'cover', backgroundPosition: 'center', minHeight: '200px', border: "0px solid blue" }}>
        
          {/* Display the total deposited value */}
          <div className="text-center mb-4">
            <h2 className="text-4xl font-bold text-white">$1,456</h2>
            <p className="text-white">Total Deposited Value</p>
          </div>

          {/* Chart */}
          <div className="w-full h-40 flex justify-center w-full px-4"> 
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={data}>
                <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.1)" />
                <XAxis dataKey="name" stroke="rgba(255, 255, 255, 0.7)" />
                {/* <YAxis stroke="rgba(255, 255, 255, 0.7)" /> */}
                <YAxis hide />

                <Tooltip />
                <Line type="monotone" dataKey="value" stroke="#8884d8" strokeWidth={2} />
                <YAxis hide />

              </LineChart>
            </ResponsiveContainer>
          </div>

        </div>

        {/* Second column: RugSafe + Filters + Manage Vaults (75% width) */}
        <div className="hero-content ml-8 flex-none md:w-3/4 gradient-bg rounded-lg" style={{ paddingTop: "40px", minHeight: '250px', flexGrow: 1 }}>
          <div className="text-center md:text-left" style={{ border: "0px solid green", flexGrow: 1 }}>
            <h1 className="text-5xl font-bold">{title}</h1>
            <p className="py-6">{subtitle}</p>

            {/* Filters Section */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4" style={{ border: "0px solid orange" }}>
              {/* Search Filter */}
              <div className="mb-4">
                <label className="block text-white text-sm font-bold mb-2" htmlFor="search">
                  Search
                </label>
                <div className="flex items-center border rounded-lg px-3 py-2 bg-gray-800">
                  <input
                    className="appearance-none bg-transparent border-none w-full text-gray-300 py-1 px-2 leading-tight focus:outline-none"
                    id="search"
                    type="text"
                    placeholder="Search term..."
                  />
                  <button className="text-gray-400 hover:text-white">
                    <svg className="w-5 h-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 11a4 4 0 118 0 4 4 0 01-8 0zm-4 8l4-4" />
                    </svg>
                  </button>
                </div>
              </div>

              {/* Blockchain Filter */}
              <div className="mb-4">
                <label className="block text-white text-sm font-bold mb-2" htmlFor="blockchain">
                  Select Blockchain
                </label>
                <div className="relative">
                  <select className="block appearance-none w-full bg-gray-800 border border-gray-600 text-gray-300 py-2 px-3 pr-8 rounded leading-tight focus:outline-none focus:bg-gray-700"
                          id="blockchain">
                    <option>All</option>
                    <option>Ethereum</option>
                    <option>Solana</option>
                    <option>Binance Smart Chain</option>
                  </select>
                  <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-300">
                    <svg className="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                      <path d="M7 10l5 5 5-5H7z" />
                    </svg>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Manage Vaults Section */}
          <CreateVault />
        </div>

                


      </div>
    </div>
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
