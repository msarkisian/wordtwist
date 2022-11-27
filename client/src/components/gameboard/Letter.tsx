import React from 'react';

interface LetterProps {
  letter: string;
  // onHover, onClick, etc
}

export const Letter: React.FC<LetterProps> = ({ letter }) => {
  return <div>{letter}</div>;
};
