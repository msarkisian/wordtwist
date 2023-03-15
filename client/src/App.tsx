import { useState } from 'react';
import { GameBoard } from './components/gameboard/GameBoard';
import { Header } from './components/Header';
import { LoginRegister } from './components/login/LoginRegister';
import { useLocalStorageState } from './hooks/useLocalStorageState';

function App() {
  const [username, setUsername] = useLocalStorageState<string | null>(
    'user',
    null
  );
  const [showLogin, setShowLogin] = useState(false);

  const login = async (username: string, password: string) => {
    const loginUrl = '/login';
    const body = { username, password };

    const res = await fetch(loginUrl, {
      method: 'POST',
      headers: {
        'Content-type': 'application/json',
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      // TODO pass error to component
      return;
    }
    setUsername(username);
  };

  return (
    <>
      <Header username={username} setShowLogin={setShowLogin} />
      {showLogin ? <LoginRegister login={login} /> : <GameBoard />}
    </>
  );
}

export default App;
