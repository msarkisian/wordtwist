import React from 'react';

interface LetterProps {
  letter: string;
  selected: boolean;
  handleMouseDown: React.MouseEventHandler;
  handleMouseOver: React.MouseEventHandler;

  // onHover, onClick, etc
}

export const Letter: React.FC<LetterProps> = ({
  letter,
  selected,
  handleMouseDown,
  handleMouseOver,
}) => {
  return (
    <div
      className="gameLetter"
      onMouseDown={handleMouseDown}
      onMouseOver={handleMouseOver}
    >
      {letter.toUpperCase()}
    </div>
  );
};
