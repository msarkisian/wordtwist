import React, { useEffect, useRef, useState } from 'react';
import { Letter } from './Letter';
import './GameBoard.css';
import { GameData, GameGrid } from '../../@types';
import { GameOptions } from './GameOptions';
import { useLocalStorageState } from '../../hooks/useLocalStorageState';
import { GameResults } from './GameResults';

interface GameBoardProps {}

export const GameBoard: React.FC<GameBoardProps> = ({}) => {
  const [grid, setGrid] = useState<GameGrid | null>(null);
  const [foundWords, setFoundWords] = useState<string[]>([]);
  const [score, setScore] = useState(0);
  const [remainingTime, setRemainingTime] = useState<number>(120);
  const timerIntervalRef = useRef<number | undefined>(undefined);
  // not setting this to falses on initialization to account for different game sizes
  const [selectedLetters, setSelectedLetters] = useState<boolean[][] | null>(
    null
  );
  const [selectedWord, setSelectedWord] = useState('');
  // this is to allow backtracking to remove letters from the selection
  const [letterPath, setLetterPath] = useState<[number, number][]>([]);
  const [validWords, setValidWords] = useState<string[]>([]);
  const [preGame, setPreGame] = useState(true);
  const [postGame, setPostGame] = useState(false);

  // generational state:
  const [size, setSize] = useLocalStorageState('lastSize', 5);
  const [lastTime, setLastTime] = useLocalStorageState('lastTime', 120);

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
      setScore((s) => s + 2 ** selectedWord.length);
    }
    setSelectedWord('');
    setSelectedLetters(Array(n).fill(Array(n).fill(false)));
    setLetterPath([]);
  };

  const startGame = async () => {
    const res = await fetch(`/game/${size}`);
    const jsonRes: GameData = await res.json();
    setGrid(jsonRes.data.grid);
    setValidWords(jsonRes.data.valid_words);
    setSelectedLetters(
      Array(jsonRes.data.grid.length).fill(
        Array(jsonRes.data.grid.length).fill(false)
      )
    );
    startTimer();
    setPreGame(false);
  };

  const startTimer = () => {
    timerIntervalRef.current = setInterval(() => {
      setRemainingTime((t) => t! - 1);
    }, 1000);
  };

  const reset = () => {
    setGrid(null);
    setFoundWords([]);
    setScore(0);
    setSelectedLetters(null);
    setSelectedWord('');
    setLetterPath([]);
    setRemainingTime(lastTime);
    setPreGame(true);
    setPostGame(false);
  };

  useEffect(() => {
    if (remainingTime === 0) {
      clearInterval(timerIntervalRef.current);
      setPostGame(true);
    }
  }, [remainingTime]);

  if (preGame) {
    return (
      <GameOptions
        remainingTime={remainingTime}
        setRemainingTime={setRemainingTime}
        setLastTime={setLastTime}
        size={size}
        setSize={setSize}
        startGame={startGame}
      />
    );
  }

  if (postGame) {
    return (
      <GameResults
        foundWords={foundWords}
        score={score}
        validWords={validWords}
        reset={reset}
      />
    );
  }

  return (
    <div className="gameContainer">
      <div
        className="gameGrid"
        style={{
          gridTemplateColumns: `${100 / grid!.length}%`.repeat(grid!.length),
        }}
        onMouseUp={() => handleMouseUp(grid!.length)}
      >
        {grid!.map((row, y) =>
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
      <div className="gameStatsContainer">
        <div>
          Time remaining:{' '}
          <strong>
            {remainingTime! > 60 ? (
              <>
                {Math.floor(remainingTime! / 60)}:
                {remainingTime! % 60 < 10
                  ? `0${remainingTime! % 60}`
                  : remainingTime! % 60}
              </>
            ) : (
              <>{remainingTime!}</>
            )}
          </strong>
        </div>
        <div>
          Score: <strong>{score}</strong>
        </div>
        Found words:
        <ul>
          {foundWords.map((word) => (
            <li key={word}>{word}</li>
          ))}
        </ul>
      </div>
    </div>
  );
};
