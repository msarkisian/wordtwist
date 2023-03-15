import React from 'react';
import { User } from '../@types';

interface HeaderProps {
  user: User | null;
}

export const Header: React.FC<HeaderProps> = ({ user }) => {
  return (
    <header className="h-12 flex bg-indigo-900 justify-between">
      <h1 className="text-white font-extrabold mx-4 my-auto">Wordtwist</h1>
      {!user ? (
        <button className="btn-secondary w-16 h-3/4 mx-4 my-auto">
          Log in
        </button>
      ) : (
        <h2 className="text-white font-semibold mx-4 my-auto">
          {user.username}
        </h2>
      )}
    </header>
  );
};
