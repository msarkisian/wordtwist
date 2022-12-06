import React from 'react';

interface GameResultsProps {
  score: number;
  foundWords: string[];
  validWords: string[];
  setValidWords: (wordlist: string[]) => void;
  reset: () => void;
}

export const GameResults: React.FC<GameResultsProps> = ({
  score,
  foundWords,
  validWords,
  setValidWords,
  reset,
}) => {
  setValidWords([...validWords].sort((a, b) => b.length - a.length));
  return (
    <div className="gameResults">
      <p>
        You scored <strong>{score}</strong> points!
      </p>
      <p>Words in this puzzle:</p>
      <ul>
        {foundWords.map((word) => (
          <li className="foundWord" key={word}>
            {word}
          </li>
        ))}
        {validWords.map((word) => (
          <li className="missedWord" key={word}>
            {word}
          </li>
        ))}
      </ul>
      <button onClick={reset}>Start new game</button>
    </div>
  );
};
