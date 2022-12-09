import React from 'react';

interface GameOptionsProps {
  remainingTime: number;
  setRemainingTime: (time: number) => void;
  setLastTime: (time: number) => void;
  size: number;
  setSize: (size: number) => void;
  startGame: () => void;
}

export const GameOptions: React.FC<GameOptionsProps> = ({
  remainingTime,
  setRemainingTime,
  setLastTime,
  size,
  setSize,
  startGame,
}) => {
  return (
    <form className="gameForm">
      <label>
        Game size:
        <input
          type="number"
          name="size"
          value={size}
          min="3"
          max="8"
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
      <input
        type={'submit'}
        value="Start game"
        onClick={(e) => {
          e.preventDefault();
          startGame();
        }}
        disabled={
          size < 3 || size > 8 || remainingTime < 10 || remainingTime > 600
        }
      />
    </form>
  );
};
