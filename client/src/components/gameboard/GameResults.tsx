import React from 'react';

interface GameResultsProps {
  score: number;
  foundWords: string[];
  validWords: string[];
  reset: () => void;
}

export const GameResults: React.FC<GameResultsProps> = ({
  score,
  foundWords,
  validWords,
  reset,
}) => {
  return (
    <div className="gameResults">
      <p>
        You scored <strong>{score}</strong> points!
      </p>
      <button onClick={reset}>Start new game</button>
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
    </div>
  );
};
