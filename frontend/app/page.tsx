'use client';

import { useState } from 'react';
import { User } from '../types/generated/User';

export default function RegisterPage() {
  const [email, setEmail] = useState('');
  const [username, setUsername] = useState('');
  const [user, setUser] = useState<User | null>(null);

  const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch(`${API_URL}/register`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: Math.random().toString(36).substring(2, 11), email, username }),
      });
      const data = await response.json();
      setUser(data);
    } catch (error) {
      console.error('Registration failed:', error);
    }
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24 bg-gray-950 text-white">
      <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm flex mb-12">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
          DDD + Hexagonal Rust Template
        </p>
      </div>

      <h1 className="text-5xl font-extrabold mb-8 bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-emerald-400">
        AI-Autonomous Registration
      </h1>
      
      <form onSubmit={handleRegister} className="flex flex-col gap-6 w-full max-w-md p-8 bg-gray-900/50 border border-gray-800 rounded-2xl backdrop-blur-sm">
        <div className="flex flex-col gap-2">
          <label className="text-sm font-medium text-gray-400">Email Address</label>
          <input
            type="email"
            placeholder="name@example.com"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            className="p-3 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
            required
          />
        </div>
        
        <div className="flex flex-col gap-2">
          <label className="text-sm font-medium text-gray-400">Username</label>
          <input
            type="text"
            placeholder="johndoe"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            className="p-3 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
            required
          />
        </div>

        <button 
          type="submit" 
          className="mt-2 p-3 bg-blue-600 hover:bg-blue-700 font-bold rounded-lg transition-all transform active:scale-95 shadow-lg shadow-blue-500/20"
        >
          Register User
        </button>
      </form>

      {user && (
        <div className="mt-12 p-8 bg-emerald-900/20 border border-emerald-500/30 rounded-2xl animate-in fade-in slide-in-from-bottom-4 duration-500">
          <h2 className="text-2xl font-bold text-emerald-400 mb-4 flex items-center gap-2">
            <span className="flex h-3 w-3 rounded-full bg-emerald-500 animate-pulse"></span>
            Registration Successful!
          </h2>
          <div className="bg-black/40 p-4 rounded-lg border border-emerald-500/10">
            <pre className="text-sm font-mono text-emerald-200 overflow-auto max-w-sm">
              {JSON.stringify(user, null, 2)}
            </pre>
          </div>
        </div>
      )}
    </main>
  );
}
