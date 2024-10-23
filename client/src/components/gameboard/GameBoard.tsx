import React, { useContext, useEffect, useRef, useState } from 'react';
import { Letter } from './Letter';
import { GameData, GameGrid, SocketResponse } from '../../@types';
import { GameOptions } from './GameOptions';
import { useLocalStorageState } from '../../hooks/useLocalStorageState';
import { GameResults } from './GameResults';
import UserContext from '../../UserContext';

interface GameBoardProps {}

export const GameBoard: React.FC<GameBoardProps> = ({}) => {
  // generational state:
  const [gameId, setGameId] = useState<string | null>(null);
  const [size, setSize] = useLocalStorageState('lastSize', 5);
  const [lastTime, setLastTime] = useLocalStorageState('lastTime', 120);
  const [error, setError] = useState<string | null>(null);
  const errorTimeout = useRef<number | undefined>(undefined);

  const [grid, setGrid] = useState<GameGrid | null>(null);
  const [foundWords, setFoundWords] = useState<string[]>([]);
  const [score, setScore] = useState(0);
  const [remainingTime, setRemainingTime] = useState<number>(lastTime);
  const timerIntervalRef = useRef<Timer | undefined>(undefined);
  // not setting this to falses on initialization to account for different game sizes
  const [selectedLetters, setSelectedLetters] = useState<boolean[][] | null>(
    null
  );
  const [selectedWord, setSelectedWord] = useState('');
  // this is to allow backtracking to remove letters from the selection
  const [letterPath, setLetterPath] = useState<[number, number][]>([]);
  const [preGame, setPreGame] = useState(true);
  const [postGame, setPostGame] = useState(false);
  const [missedWords, setMissedWords] = useState<string[]>([]);
  const [endgameMessage, setEndgameMessage] = useState<string | null>('');

  const socket = useRef<WebSocket | null>(null);

  const username = useContext(UserContext);

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
    socket.current!.send(selectedWord);
    setSelectedWord('');
    setSelectedLetters(Array(n).fill(Array(n).fill(false)));
    setLetterPath([]);
  };

  const startGame = async (
    daily: boolean,
    id: string,
    size: number,
    time?: number
  ) => {
    let url: string;
    const queryString = time
      ? new URLSearchParams({ time: time.toString() }).toString()
      : '';

    if (daily) url = '/game/daily';
    else if (id !== '') url = `/game/id/${id}?${queryString}`;
    else url = `/game/${size}?${queryString}`;
    handleSocket(window.location.host + url);
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

  const handleSocket = (url: string) => {
    if (socket.current !== null) throw new Error('already have a socket open');
    socket.current = new WebSocket('ws://' + url);
    socket.current.onmessage = (event) => {
      const msg: SocketResponse = JSON.parse(event.data);
      switch (msg.type) {
        case 'setup':
          setGameId(msg.game.id);
          setGrid(msg.game.grid);
          setSelectedLetters(
            Array(msg.game.grid.length).fill(
              Array(msg.game.grid.length).fill(false)
            )
          );
          setRemainingTime(msg.time);
          setLastTime(msg.time);
          startTimer();
          setPreGame(false);
          break;
        case 'guessResponse':
          if (msg.valid) {
            setFoundWords((w) => [...w, msg.word]);
            setScore((s) => s + 2 ** msg.word.length);
          }
          break;
        case 'gameOver':
          clearInterval(timerIntervalRef.current);
          setFoundWords(msg.results.foundWords);
          setMissedWords(msg.results.missedWords);
          setEndgameMessage(msg.msg);
          socket.current = null;
          setPostGame(true);
          break;
        default:
          throw new Error('unexpected socket message: ' + msg);
      }
    };
  };

  if (preGame) {
    return (
      <GameOptions
        remainingTime={remainingTime}
        size={size}
        setSize={setSize}
        startGame={startGame}
        error={error}
      />
    );
  }

  if (postGame) {
    return (
      <GameResults
        gameId={gameId!}
        foundWords={foundWords}
        score={score}
        missedWords={missedWords}
        endgameMessage={endgameMessage}
        reset={reset}
      />
    );
  }

  return (
    <div className="flex flex-col justify-center md:flex-row m-2 mt-8">
      <div className="mr-8">
        <div
          className="grid"
          style={{
            gridTemplateColumns: '6rem '.repeat(grid!.length),
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
        </div>
        <div>selected word: {selectedWord}</div>
      </div>
      <div className="w-48">
        <div>
          Time remaining:{' '}
          <strong>
            {remainingTime! > 59 ? (
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
        <ul className="list-disc list-inside">
          {foundWords.map((word) => (
            <li className="text-sm" key={word}>
              {word}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};
