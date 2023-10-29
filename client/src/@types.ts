export type GameGrid = string[][];

export type GameData = {
  id: string;
  grid: string[][];
};

export type SocketResponse = GuessReponse | GameResults | Setup;

type GuessReponse = {
  type: 'guessResponse';
  word: string;
  valid: boolean;
};

type GameResults = {
  type: 'gameOver';
  results: {
    foundWords: string[];
    missedWords: string[];
    score: number;
  };
};

type Setup = {
  type: 'setup';
  time: number;
  game: GameData;
};
