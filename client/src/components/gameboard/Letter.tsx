import React from 'react';

interface LetterProps {
  letter: string;
  selected: boolean;
  handleMouseDown: React.MouseEventHandler;

  // onHover, onClick, etc
}

export const Letter: React.FC<LetterProps> = ({
  letter,
  selected,
  handleMouseDown,
}) => {
  return (
    <div className="gameLetter" onMouseDown={handleMouseDown}>
      {letter.toUpperCase()}
    </div>
  );
};
