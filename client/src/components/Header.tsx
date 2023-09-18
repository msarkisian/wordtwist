import React from 'react';

interface HeaderProps {
  username: string | null;
  setShowLogin: (show: boolean) => void;
  setShowRegister: (show: boolean) => void;
  logout: () => void;
}

export const Header: React.FC<HeaderProps> = ({
  username,
  setShowLogin,
  setShowRegister,
  logout,
}) => {
  return (
    <header className="h-12 flex bg-indigo-900 justify-between">
      <h1 className="text-white font-extrabold mx-4 my-auto">Wordtwist</h1>
      {!username ? (
        <div className="flex">
          <button
            className="btn-secondary w-20 h-3/4 mx-4 my-auto"
            onClick={() => {
              setShowLogin(true);
              setShowRegister(false);
            }}
          >
            Log in
          </button>
          <button
            className="btn-secondary w-20 h-3/4 mx-4 my-auto"
            onClick={() => {
              setShowLogin(true);
              setShowRegister(true);
            }}
          >
            Register
          </button>
        </div>
      ) : (
        <div className="flex">
          <h2 className="text-white font-semibold mx-4 my-auto">{username}</h2>
          <button
            className="btn-tertiary h-8 w-20 my-auto mr-2"
            onClick={logout}
          >
            Log out
          </button>
        </div>
      )}
    </header>
  );
};
