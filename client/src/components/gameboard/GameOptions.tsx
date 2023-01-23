import React, { useState } from 'react';

interface GameOptionsProps {
  remainingTime: number;
  setRemainingTime: (time: number) => void;
  setLastTime: (time: number) => void;
  size: number;
  setSize: (size: number) => void;
  startGame: (id: null | string) => void;
}

export const GameOptions: React.FC<GameOptionsProps> = ({
  remainingTime,
  setRemainingTime,
  setLastTime,
  size,
  setSize,
  startGame,
}) => {
  const [loadingGameFromId, setLoadingGameFromId] = useState(false);
  const [gameId, setGameId] = useState<string | null>(null);
  return (
    <>
      <h2 className="mx-2 text-xl font-semibold">Game options:</h2>
      <form className="flex flex-col mx-2 px-2 max-w-md bg-gray-100 border border-gray-400 rounded-md">
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
            value={remainingTime}
            min="10"
            max="600"
            onChange={(e) => {
              setRemainingTime(Number(e.target.value));
              setLastTime(Number(e.target.value));
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
              checked={!loadingGameFromId}
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
        <div className="flex justify-center">
          <input
            className="w-28 h-12 mt-6 mb-2 bg-indigo-700 text-white rounded-lg cursor-pointer hover:bg-indigo-500 active:bg-indigo-400"
            type={'submit'}
            value="Start game"
            onClick={(e) => {
              e.preventDefault();
              startGame(gameId);
            }}
            disabled={
              size < 3 || size > 7 || remainingTime < 10 || remainingTime > 600
            }
          />
        </div>
      </form>
    </>
  );
};
