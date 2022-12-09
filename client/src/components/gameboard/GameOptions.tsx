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
    <form className="gameForm">
      <label>
        Game size:
        <input
          type="number"
          name="size"
          value={size}
          min="3"
          max="7"
          onChange={(e) => setSize(Number(e.target.value))}
        />
      </label>
      <label>
        Game time (sec):
        <input
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
      <div>
        <input
          type="radio"
          value="new"
          id="new"
          name="newOrId"
          onClick={() => {
            setLoadingGameFromId(false);
            setGameId(null);
          }}
        />
        <label htmlFor="new">New game</label>
        <input
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
      {loadingGameFromId && (
        <input
          placeholder="Game ID"
          value={gameId!}
          onChange={(e) => setGameId(e.target.value)}
        />
      )}
      <input
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
    </form>
  );
};
