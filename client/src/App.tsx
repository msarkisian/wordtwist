import { createContext, useState } from 'react';
import { GameBoard } from './components/gameboard/GameBoard';
import { Header } from './components/Header';
import { LoginRegister } from './components/login/LoginRegister';
import { useLocalStorageState } from './hooks/useLocalStorageState';
import { useTimeoutState } from './hooks/useTimeoutState';
import UserContext from './UserContext';

function App() {
  const [username, setUsername] = useLocalStorageState<string | null>(
    'user',
    null
  );
  const [showLogin, setShowLogin] = useState(false);
  const [showRegister, setShowRegister] = useState(false);
  const [loginError, setLoginError] = useTimeoutState<string>(10000);

  const loginUrl = '/login';
  const registerUrl = '/user';

  const login = async (username: string, password: string) => {
    const body = { username, password };

    const res = await fetch(loginUrl, {
      method: 'POST',
      headers: {
        'Content-type': 'application/json',
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      let body = await res.text();
      setLoginError(body);
      return;
    }
    setUsername(username);
    setShowLogin(false);
  };

  const register = async (username: string, password: string) => {
    const body = { username, password };

    const res = await fetch(registerUrl, {
      method: 'POST',
      headers: {
        'Content-type': 'application/json',
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      let body = await res.text();
      setLoginError(body);
      return;
    }
    setUsername(username);
    setShowRegister(false);
  };

  const logout = async () => {
    const res = await fetch(loginUrl, {
      method: 'DELETE',
    });
    if (res.ok || res.status === 401) {
      setUsername(null);
      return;
    }
    throw new Error('server errored when sending logout request: ' + res);
  };

  return (
    <>
      <UserContext.Provider value={username}>
        <Header
          username={username}
          setShowLogin={setShowLogin}
          setShowRegister={setShowRegister}
          logout={logout}
        />
        {showLogin ? (
          <LoginRegister
            login={login}
            loginError={loginError}
            register={register}
            showRegister={showRegister}
          />
        ) : (
          <GameBoard />
        )}
      </UserContext.Provider>
    </>
  );
}

export default App;
