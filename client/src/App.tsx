import { useEffect, useState } from 'react';
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

  const [gameBoardKey, setGameBoardKey] = useState(0);

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

  const register = async (
    email: string,
    username: string,
    password: string
  ) => {
    const body = { email, username, password };

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
    setShowLogin(false);
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

  const checkIfLoggedIn = async () => {
    const res = await fetch(loginUrl);
    if (res.status !== 204) setUsername(null);
  };

  const goHome = () => {
    setShowRegister(false);
    setShowLogin(false);
    setGameBoardKey((v) => v + 1);
  };

  useEffect(() => {
    checkIfLoggedIn();
  }, []);

  return (
    <>
      <UserContext.Provider value={username}>
        <Header
          username={username}
          setShowLogin={setShowLogin}
          setShowRegister={setShowRegister}
          logout={logout}
          goHome={goHome}
        />
        {showLogin ? (
          <LoginRegister
            login={login}
            loginError={loginError}
            register={register}
            showRegister={showRegister}
          />
        ) : (
          <GameBoard key={gameBoardKey} />
        )}
      </UserContext.Provider>
    </>
  );
}

export default App;
