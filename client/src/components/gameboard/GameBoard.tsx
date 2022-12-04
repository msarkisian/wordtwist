import React, { useEffect, useState } from 'react';
import { Letter } from './Letter';
import './GameBoard.css';
import { GameData, GameGrid } from '../../@types';

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
  const [validWords, setValidWords] = useState<string[]>([]);

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

  const handleMouseUp = (n: number) => {
    if (validWords.includes(selectedWord)) {
      setFoundWords([...foundWords, selectedWord]);
      setValidWords(validWords.filter((word) => word !== selectedWord));
    }
    setSelectedWord('');
    setSelectedLetters(Array(n).fill(Array(n).fill(false)));
    setLetterPath([]);
  };

  if (!grid) {
    return (
      <button
        onClick={() => {
          fetch('/game')
            .then((res) => res.json())
            .then((jsonRes: GameData) => {
              setGrid(jsonRes.data.grid);
              setValidWords(jsonRes.data.valid_words);
              setSelectedLetters(
                Array(jsonRes.data.grid.length).fill(
                  Array(jsonRes.data.grid.length).fill(false)
                )
              );
              // window.addEventListener('mouseup', () =>
              //   handleMouseUp(jsonRes.data.grid.length)
              // );
            });
        }}
      >
        Load new game
      </button>
    );
  }

  return (
    <div className="gameContainer">
      <div
        className="gameGrid"
        style={{
          gridTemplateColumns: '100px '.repeat(grid.length),
        }}
        onMouseUp={() => handleMouseUp(grid.length)}
      >
        {grid.map((row, y) =>
          row.map((column, x) => (
            <Letter
              key={`${x},${y}`}
              letter={column}
              selected={selectedLetters![y][x]}
              isNewestLetter={
                letterPath[0] &&
                letterPath[letterPath.length - 1][0] === y &&
                letterPath[letterPath.length - 1][1] === x
              }
              handleMouseDown={() => handleMouseDown(y, x)}
              handleMouseOver={() => handleMouseOver(y, x)}
            />
          ))
        )}
        <div>selected word: {selectedWord}</div>
      </div>
      {foundWords.length > 0 && (
        <div className="foundWordsContainer">
          Found words:
          <ul>
            {foundWords.map((word) => (
              <li key={word}>{word}</li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
};
