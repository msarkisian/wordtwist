import React, { useState } from 'react';

interface GameResultsProps {
  gameId: string;
  score: number;
  foundWords: string[];
  validWords: string[];
  reset: () => void;
}

export const GameResults: React.FC<GameResultsProps> = ({
  gameId,
  score,
  foundWords,
  validWords,
  reset,
}) => {
  const [copiedId, setCopiedId] = useState(false);
  return (
    <div className="gameResults">
      <p>
        You scored <strong>{score}</strong> points!
      </p>
      <button onClick={reset}>Start new game</button>
      <button
        disabled={copiedId}
        onClick={() => {
          navigator.clipboard.writeText(gameId);
          setCopiedId(true);
        }}
      >
        {copiedId ? <>ID copied!</> : <>Copy Game ID</>}
      </button>
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
