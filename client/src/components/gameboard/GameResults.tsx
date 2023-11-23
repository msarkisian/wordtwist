import React, { useEffect, useState } from 'react';

interface GameResultsProps {
  gameId: string;
  score: number;
  foundWords: string[];
  missedWords: string[];
  endgameMessage: string | null;
  reset: () => void;
}

export const GameResults: React.FC<GameResultsProps> = ({
  gameId,
  score,
  foundWords,
  missedWords,
  endgameMessage,
  reset,
}) => {
  const [copiedId, setCopiedId] = useState(false);

  return (
    <div className="m-2">
      <h2 className="text-lg mb-2">
        You scored <strong>{score}</strong> points!
      </h2>
      <button className="btn-primary w-36 h-8 mr-2" onClick={reset}>
        Start new game
      </button>
      <button
        className={`btn-secondary w-28 h-8
        ${copiedId ? 'cursor-default' : 'cursor-pointer'}
        `}
        disabled={copiedId}
        onClick={() => {
          navigator.clipboard.writeText(gameId);
          setCopiedId(true);
        }}
      >
        {copiedId ? <>ID copied!</> : <>Copy Game ID</>}
      </button>
      <h3>{endgameMessage && endgameMessage}</h3>
      <p className="mt-4">Words in this puzzle:</p>
      <ul className="list-disc mx-6">
        {foundWords.map((word) => (
          <li className="text-green-800 " key={word}>
            {word}
          </li>
        ))}
        {missedWords.map((word) => (
          <li className="text-red-800" key={word}>
            {word}
          </li>
        ))}
      </ul>
    </div>
  );
};
