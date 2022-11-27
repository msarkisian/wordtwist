import React, { useState } from 'react';
import { Letter } from './Letter';
import './GameBoard.css';
import { GameGrid } from '../../@types';

interface GameBoardProps {}

export const GameBoard: React.FC<GameBoardProps> = ({}) => {
  const [grid, setGrid] = useState<GameGrid | null>(null);
  const [foundWords, setFoundWords] = useState<string[]>([]);
  const [score, setScore] = useState(0);
  // not setting this to falses on initialization to account for different game sizes
  const [selectedLetters, setSelectedLetters] = useState<boolean[][] | null>(
    null
  );
  const [selectedWord, setSelectedWord] = useState('');

  const handleMouseDown = (y: number, x: number) => {
    console.log(y, x);
  };

  const dummyGrid = [
    ['f', 'e', 'b', 'r', 'n'],
    ['n', 'o', 'e', 'b', 'h'],
    ['n', 'v', 'r', 'e', 'g'],
    ['t', 'e', 'm', 'c', 'b'],
    ['r', 'f', 'y', 'l', 'v'],
  ];

  if (!grid) {
    return (
      <button
        onClick={() => {
          setGrid(dummyGrid);
          setSelectedLetters(Array(5).fill(Array(5).fill(false)));
        }}
      >
        load dummy grid
      </button>
    );
  }

  return (
    <div
      className="gameGrid"
      style={{
        gridTemplateColumns: '100px '.repeat(dummyGrid.length),
      }}
    >
      {grid.map((row, y) =>
        row.map((column, x) => (
          <Letter
            letter={column}
            selected={selectedLetters![y][x]}
            handleMouseDown={() => handleMouseDown(y, x)}
          />
        ))
      )}
    </div>
  );
};
