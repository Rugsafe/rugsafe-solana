"use client"
import React, { useState } from 'react';
import { AdvancedRealTimeChart } from "react-ts-tradingview-widgets";
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';

interface Order {
  id: string;
  symbol: string;
  side: 'Long' | 'Short';
  size: string;
  leverage: string;
  entryPrice: number;
  markPrice: number;
  pnl: number;
}

interface NewOrder {
  symbol: string;
  side: 'Long' | 'Short';
  size: string;
  leverage: string;
}

interface PnlData {
  time: string;
  pnl: number;
}

// Mock data for the orders
const initialOrders: Order[] = [
  { id: '1', symbol: 'BTCUSDT', side: 'Long', size: '0.5', leverage: '10', entryPrice: 30000, markPrice: 30500, pnl: 250 },
  { id: '2', symbol: 'ETHUSDT', side: 'Short', size: '5', leverage: '5', entryPrice: 2000, markPrice: 1950, pnl: 250 },
];

// Mock data for the PnL chart
const mockPnlData: PnlData[] = [
  { time: '00:00', pnl: 0 },
  { time: '04:00', pnl: 100 },
  { time: '08:00', pnl: -50 },
  { time: '12:00', pnl: 200 },
  { time: '16:00', pnl: 150 },
  { time: '20:00', pnl: 300 },
];

const TradingViewPerpsWithOrders: React.FC = () => {
  const [orders, setOrders] = useState<Order[]>(initialOrders);
  const [newOrder, setNewOrder] = useState<NewOrder>({
    symbol: 'BTCUSDT',
    side: 'Long',
    size: '',
    leverage: '',
  });

  const handleClosePosition = (id: string) => {
    setOrders(orders.filter(order => order.id !== id));
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setNewOrder(prev => ({ ...prev, [name]: value }));
  };

  const handleSubmitOrder = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const markPrice = newOrder.symbol === 'BTCUSDT' ? 30000 : 2000; // Mock current price
    const newPosition: Order = {
      id: Date.now().toString(),
      ...newOrder,
      entryPrice: markPrice,
      markPrice: markPrice,
      pnl: 0,
    };
    setOrders(prev => [...prev, newPosition]);
    setNewOrder({ symbol: 'BTCUSDT', side: 'Long', size: '', leverage: '' });
  };

  return (
    <div className="flex flex-col h-screen bg-gray-900 text-white">
      <div className="flex-1 min-h-0">
        <AdvancedRealTimeChart
          theme="dark"
          autosize
          symbol="BINANCE:BTCUSDT.P"
          interval="D"
          timezone="Etc/UTC"
          style="1"
          locale="en"
          toolbar_bg="#f1f3f6"
          enable_publishing={false}
          allow_symbol_change={true}
          container_id="tradingview_chart"
        />
      </div>
      <div className="h-1/2 p-4 overflow-auto">
        <h2 className="text-xl font-bold mb-4">Open New Position</h2>
        <form onSubmit={handleSubmitOrder} className="mb-6 flex space-x-4">
          <select
            name="symbol"
            value={newOrder.symbol}
            onChange={handleInputChange}
            className="bg-gray-700 text-white px-3 py-2 rounded"
          >
            <option value="BTCUSDT">BTCUSDT</option>
            <option value="ETHUSDT">ETHUSDT</option>
          </select>
          <select
            name="side"
            value={newOrder.side}
            onChange={handleInputChange}
            className="bg-gray-700 text-white px-3 py-2 rounded"
          >
            <option value="Long">Long</option>
            <option value="Short">Short</option>
          </select>
          <input
            type="number"
            name="size"
            value={newOrder.size}
            onChange={handleInputChange}
            placeholder="Size"
            className="bg-gray-700 text-white px-3 py-2 rounded"
            required
          />
          <input
            type="number"
            name="leverage"
            value={newOrder.leverage}
            onChange={handleInputChange}
            placeholder="Leverage"
            className="bg-gray-700 text-white px-3 py-2 rounded"
            required
          />
          <button type="submit" className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600">
            Open Position
          </button>
        </form>

        <h2 className="text-xl font-bold mb-4">Open Positions</h2>
        <table className="w-full text-sm text-left">
          <thead className="text-xs uppercase bg-gray-700">
            <tr>
              <th className="px-6 py-3">Symbol</th>
              <th className="px-6 py-3">Side</th>
              <th className="px-6 py-3">Size</th>
              <th className="px-6 py-3">Leverage</th>
              <th className="px-6 py-3">Entry Price</th>
              <th className="px-6 py-3">Mark Price</th>
              <th className="px-6 py-3">PNL</th>
              <th className="px-6 py-3">Action</th>
            </tr>
          </thead>
          <tbody>
            {orders.map((order) => (
              <tr key={order.id} className="border-b bg-gray-800 border-gray-700">
                <td className="px-6 py-4">{order.symbol}</td>
                <td className="px-6 py-4">{order.side}</td>
                <td className="px-6 py-4">{order.size}</td>
                <td className="px-6 py-4">{order.leverage}x</td>
                <td className="px-6 py-4">${order.entryPrice}</td>
                <td className="px-6 py-4">${order.markPrice}</td>
                <td className="px-6 py-4">${order.pnl}</td>
                <td className="px-6 py-4">
                  <button 
                    onClick={() => handleClosePosition(order.id)}
                    className="font-medium text-blue-500 hover:underline"
                  >
                    Close
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        
        <h2 className="text-xl font-bold my-4">PNL Chart</h2>
        <div className="h-64">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={mockPnlData}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="time" />
              <YAxis />
              <Tooltip />
              <Legend />
              <Line type="monotone" dataKey="pnl" stroke="#8884d8" />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>
    </div>
  );
};

export default TradingViewPerpsWithOrders;