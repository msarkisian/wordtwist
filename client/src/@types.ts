export type GameGrid = string[][];

export type GameData = {
  id: string;
  data: {
    grid: string[][];
    valid_words: string[];
  };
};
