import React from 'react';
import { Letter } from './Letter';
import './GameBoard.css';

interface GameBoardProps {}

export const GameBoard: React.FC<GameBoardProps> = ({}) => {
  const dummyGrid = [
    ['f', 'e', 'b', 'r', 'n'],
    ['n', 'o', 'e', 'b', 'h'],
    ['n', 'v', 'r', 'e', 'g'],
    ['t', 'e', 'm', 'c', 'b'],
    ['r', 'f', 'y', 'l', 'v'],
  ];
  return (
    <div
      className="gameGrid"
      style={{
        gridTemplateColumns: '1fr '.repeat(dummyGrid.length),
      }}
    >
      {dummyGrid.map((row) => row.map((column) => <Letter letter={column} />))}
    </div>
  );
};
