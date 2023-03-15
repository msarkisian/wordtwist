import { useState } from 'react';
import { User } from './@types';
import { GameBoard } from './components/gameboard/GameBoard';
import { Header } from './components/Header';
import { LoginRegister } from './components/login/LoginRegister';
import { useLocalStorageState } from './hooks/useLocalStorageState';

function App() {
  const [user, setUser] = useLocalStorageState<User | null>('user', null);
  const [showLogin, setShowLogin] = useState(false);
  return (
    <>
      <Header user={user} setShowLogin={setShowLogin} />
      {showLogin ? <LoginRegister /> : <GameBoard />}
    </>
  );
}

export default App;
