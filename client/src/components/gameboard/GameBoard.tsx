import React, { useEffect, useState } from 'react';
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
  // this is to allow backtracking to remove letters from the selection
  const [letterPath, setLetterPath] = useState<[number, number][]>([]);

  const handleMouseDown = (y: number, x: number) => {
    if (!grid) return;
    if (!selectedLetters) return;
    setSelectedWord(grid[y][x]);
    setSelectedLetters((arr) => {
      const copy = arr!.map((a) => a.slice());
      copy[y][x] = true;
      return copy;
    });
    setLetterPath([...letterPath, [y, x]]);
  };

  const handleMouseOver = (y: number, x: number) => {
    if (!selectedWord) return;
    const [lastY, lastX] = letterPath[letterPath.length - 1];
    // return if they're trying to add a letter not adjacent to their last letter
    if (Math.abs(lastY - y) > 1 || Math.abs(lastX - x) > 1) return;
    if (
      letterPath[letterPath.length - 2] &&
      letterPath[letterPath.length - 2][0] === y &&
      letterPath[letterPath.length - 2][1] === x
    ) {
      // backtrack
      setSelectedWord(selectedWord.slice(0, selectedWord.length - 1));
      setSelectedLetters((arr) => {
        if (!arr) return null;
        const copy = arr.map((a) => a.slice());
        copy[lastY][lastX] = false;
        return copy;
      });
      setLetterPath(letterPath.slice(0, letterPath.length - 1));
    } else {
      // new letter
      if (selectedLetters![y][x]) return;
      setSelectedWord(selectedWord + grid![y][x]);
      setSelectedLetters((arr) => {
        const copy = arr!.map((a) => a.slice());
        copy[y][x] = true;
        return copy;
      });
      setLetterPath([...letterPath, [y, x]]);
    }
  };

  const handleMouseUp = () => {
    setSelectedWord('');
    setSelectedLetters(Array(5).fill(Array(5).fill(false)));
    setLetterPath([]);
  };

  useEffect(() => {
    window.addEventListener('mouseup', handleMouseUp);
  }, []);

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
            key={`${x},${y}`}
            letter={column}
            selected={selectedLetters![y][x]}
            handleMouseDown={() => handleMouseDown(y, x)}
            handleMouseOver={() => handleMouseOver(y, x)}
          />
        ))
      )}
      <div>selected word: {selectedWord}</div>
    </div>
  );
};
