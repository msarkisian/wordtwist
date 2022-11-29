import React from 'react';

interface LetterProps {
  letter: string;
  selected: boolean;
  isNewestLetter: boolean;
  handleMouseDown: React.MouseEventHandler;
  handleMouseOver: React.MouseEventHandler;

  // onHover, onClick, etc
}

export const Letter: React.FC<LetterProps> = ({
  letter,
  selected,
  isNewestLetter,
  handleMouseDown,
  handleMouseOver,
}) => {
  return (
    <div
      className="gameLetter"
      onMouseDown={handleMouseDown}
      onMouseOver={handleMouseOver}
      style={{
        backgroundColor: selected ? 'yellow' : 'lightgreen',
        // hacky "grow border inside only" solution
        boxShadow: isNewestLetter ? '0px 0px 0px 3px black inset' : '',
      }}
    >
      {letter.toUpperCase()}
    </div>
  );
};
