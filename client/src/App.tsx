import { User } from './@types';
import { GameBoard } from './components/gameboard/GameBoard';
import { Header } from './components/Header';
import { useLocalStorageState } from './hooks/useLocalStorageState';

function App() {
  const [user, setUser] = useLocalStorageState<User | null>('user', null);
  return (
    <>
      <Header user={user} />
      <GameBoard />
    </>
  );
}

export default App;
