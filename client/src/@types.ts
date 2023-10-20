export type GameGrid = string[][];

export type GameData = {
  id: string;
  data: {
    grid: string[][];
    valid_words: string[];
  };
};

export type SocketResponse = GuessReponse | GameResults;

type GuessReponse = {
  type: 'guessResponse';
  word: string;
  valid: boolean;
};

type GameResults = {
  type: 'gameOver';
  foundWords: string[];
  missedWords: string[];
  score: number;
};
