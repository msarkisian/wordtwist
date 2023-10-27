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
      className={`rounded-full h-24 w-24 flex justify-center items-center text-3xl cursor-pointer select-none border border-gray-800  ${
        selected ? 'bg-indigo-300' : 'bg-yellow-200'
      }
      ${isNewestLetter && 'bg-indigo-400'}
      `}
      onMouseDown={handleMouseDown}
      onMouseOver={handleMouseOver}
      style={{
        // backgroundColor: selected ? 'yellow' : 'lightgreen',
        // hacky "grow border inside only" solution
        boxShadow: isNewestLetter ? '0px 0px 0px 3px black inset' : '',
      }}
    >
      {letter.toUpperCase()}
    </div>
  );
};
