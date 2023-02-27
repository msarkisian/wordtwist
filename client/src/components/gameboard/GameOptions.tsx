import React, { useState } from 'react';

interface GameOptionsProps {
  remainingTime: number;
  size: number;
  setSize: (size: number) => void;
  startGame: (
    daily: boolean,
    id: null | string,
    size: number,
    time: number
  ) => void;
  error: string | null;
}

export const GameOptions: React.FC<GameOptionsProps> = ({
  remainingTime,
  size,
  setSize,
  startGame,
  error,
}) => {
  const [loadingGameFromId, setLoadingGameFromId] = useState(false);
  const [time, setTime] = useState(remainingTime);
  const [gameId, setGameId] = useState<string | null>(null);
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
              disabled={loadingGameFromId}
              value={size}
              min="3"
              max="7"
              onChange={(e) => setSize(Number(e.target.value))}
            />
          </label>
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
          <div className="flex justify-around">
            <div>
              <input
                className="mr-1"
                type="radio"
                value="new"
                id="new"
                name="newOrId"
                defaultChecked
                onClick={() => {
                  setLoadingGameFromId(false);
                  setGameId(null);
                }}
              />
              <label htmlFor="new">New game</label>
            </div>
            <div>
              <input
                className="mr-1"
                type="radio"
                value="fromId"
                id="fromId"
                name="newOrId"
                onClick={() => {
                  setLoadingGameFromId(true);
                  setGameId('');
                }}
              />
              <label htmlFor="fromId">Load from Game ID</label>
            </div>
          </div>
          {loadingGameFromId && (
            <div className="flex justify-center mt-2">
              <input
                className="w-5/6"
                placeholder="Game ID"
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
            <input
              className="w-40 h-12 m-2 bg-indigo-700 text-white rounded-lg cursor-pointer hover:bg-indigo-500 active:bg-indigo-400 disabled:bg-indigo-300 disabled:cursor-default"
              type={'submit'}
              value={loadingGameFromId ? 'Load game' : 'Create new game'}
              onClick={(e) => {
                e.preventDefault();
                startGame(false, gameId, size, time);
              }}
              disabled={
                size < 3 ||
                size > 7 ||
                remainingTime < 10 ||
                remainingTime > 600 ||
                (loadingGameFromId && !gameId)
              }
            />
            {!loadingGameFromId && (
              <input
                className="w-48 h-12 m-2 bg-indigo-700 text-white rounded-lg cursor-pointer hover:bg-indigo-500 active:bg-indigo-400 disabled:bg-indigo-300 disabled:cursor-default"
                type={'submit'}
                value="Play today's daily game"
                onClick={(e) => {
                  e.preventDefault();
                  startGame(true, null, 4, remainingTime);
                }}
                disabled={remainingTime < 10 || remainingTime > 600}
              />
            )}
          </div>
        </form>
      </div>
    </>
  );
};
