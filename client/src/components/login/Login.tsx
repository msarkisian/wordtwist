import React from 'react';

interface LoginProps {}

export const Login: React.FC<LoginProps> = ({}) => {
  return (
    <form className="flex flex-col w-2/3 max-w-md bg-gray-200 py-2 border border-gray-400 rounded-md">
      <h2 className="text-xl text-center font-bold">Log in:</h2>
      <label className="my-2 flex justify-evenly">
        <span className="block">Username:</span>
        <input />
      </label>
      <label className="my-2 flex justify-evenly">
        <span className="block">Password:</span>
        <input type={'password'} />
      </label>
      <input
        className="btn-primary w-20 h-8 m-auto"
        type={'submit'}
        value="Log in"
      />
    </form>
  );
};
