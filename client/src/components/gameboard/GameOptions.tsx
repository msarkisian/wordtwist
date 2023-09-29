import React, { useState } from 'react';

interface GameOptionsProps {
  remainingTime: number;
  size: number;
  setSize: (size: number) => void;
  startGame: (daily: boolean, id: string, size: number, time: number) => void;
  error: string | null;
}

// const defaultTimes = {
//   3: 60,
//   4: 120,
//   5: 180,
//   6: 240,
//   7: 300,
// };

export const GameOptions: React.FC<GameOptionsProps> = ({
  remainingTime,
  size,
  setSize,
  startGame,
  error,
}) => {
  const [advancedSettings, setAdvancedSettings] = useState(false);
  const [time, setTime] = useState(remainingTime);
  const [gameId, setGameId] = useState<string>('');
  return (
    <>
      <h2 className="m-2 text-xl font-semibold text-center">Game options:</h2>
      <div className="flex justify-center">
        <form className="flex flex-col mx-2 px-2 w-96 bg-gray-100 border border-gray-400 rounded-md">
          <label className="my-2 flex justify-evenly">
            <span className="inline-block">Game size:</span>
            <input
              className="w-16"
              type="number"
              name="size"
              value={size}
              min="3"
              max="7"
              onChange={(e) => setSize(Number(e.target.value))}
            />
          </label>
          {advancedSettings && (
            <label className="my-2 flex justify-evenly">
              <span className="inline-block">Game time (sec):</span>
              <input
                className="w-24"
                type="number"
                name="time"
                value={time}
                min="10"
                max="600"
                onChange={(e) => {
                  setTime(Number(e.target.value));
                }}
              />
            </label>
          )}

          {advancedSettings && (
            <div className="flex justify-center mt-2">
              <input
                className="w-5/6"
                placeholder="Game ID (optional)"
                value={gameId!}
                onChange={(e) => setGameId(e.target.value)}
              />
            </div>
          )}
          {error && (
            <p className="text-red-600 font-semibold text-center">
              Error: {error}
            </p>
          )}
          <div className="flex justify-center">
            {/* "basic game" start button, ignores `time` and uses default */}
            {!advancedSettings && (
              <input
                className="w-40 h-12 m-2 bg-indigo-700 text-white rounded-lg cursor-pointer hover:bg-indigo-500 active:bg-indigo-400 disabled:bg-indigo-300 disabled:cursor-default"
                type={'submit'}
                value="Create new game"
                onClick={(e) => {
                  e.preventDefault();
                  if (size < 3 || size > 7) return;
                  startGame(false, gameId, size, (size - 1) * 30);
                }}
              />
            )}
            {!advancedSettings && (
              <input
                className="w-48 h-12 m-2 bg-indigo-700 text-white rounded-lg cursor-pointer hover:bg-indigo-500 active:bg-indigo-400 disabled:bg-indigo-300 disabled:cursor-default"
                type={'submit'}
                value="Play daily game"
                onClick={(e) => {
                  e.preventDefault();
                  startGame(true, '', 4, remainingTime);
                }}
                disabled={remainingTime < 10 || remainingTime > 600}
              />
            )}
            {/* "advanced game" start button, uses provided `time` and game id*/}
            {advancedSettings && (
              <input
                className="w-40 h-12 m-2 bg-indigo-700 text-white rounded-lg cursor-pointer hover:bg-indigo-500 active:bg-indigo-400 disabled:bg-indigo-300 disabled:cursor-default"
                type={'submit'}
                value={gameId === '' ? 'Create new game' : 'Load game from ID'}
                onClick={(e) => {
                  e.preventDefault();
                  if (size < 3 || size > 7) return;
                  startGame(false, gameId, size, time);
                }}
              />
            )}
          </div>
          <div className="flex justify-center">
            <button
              className="btn-secondary w-48 h-8 my-2"
              onClick={(e) => {
                e.preventDefault();
                setGameId('');
                setAdvancedSettings(!advancedSettings);
              }}
            >
              {advancedSettings ? <>Basic options</> : <>Advanced options</>}
            </button>
          </div>
        </form>
      </div>
    </>
  );
};
