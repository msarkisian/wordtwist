import React from 'react';

interface LoginProps {
  login: (username: string, password: string) => void;
  register: (email: string, username: string, password: string) => void;
  loginError: string | null;
  showRegister: boolean;
}

export const Login: React.FC<LoginProps> = ({
  login,
  loginError,
  register,
  showRegister,
}) => {
  return (
    <form
      className="flex flex-col w-2/3 max-w-md bg-gray-200 py-2 border border-gray-400 rounded-md"
      onSubmit={(e) => {
        e.preventDefault();
        const target = e.target as HTMLFormElement;
        if (!showRegister) login(target.username.value, target.password.value);
        else
          register(
            target.email.value,
            target.username.value,
            target.password.value
          );
      }}
    >
      <h2 className="text-xl text-center font-bold">
        {showRegister ? <>Create new account:</> : <>Log in:</>}
      </h2>
      {showRegister && (
        <label className="my-2 flex justify-evenly">
          <span className="block">Email address:</span>
          <input name="email" />
        </label>
      )}
      <label className="my-2 flex justify-evenly">
        <span className="block">Username:</span>
        <input name="username" />
      </label>
      <label className="my-2 flex justify-evenly">
        <span className="block">Password:</span>
        <input type="password" name="password" />
      </label>
      <input
        className="btn-primary w-20 h-8 m-auto"
        type="submit"
        value={showRegister ? 'Register' : 'Log in'}
      />
      {loginError && <p className="m-auto mt-4 text-red-800">{loginError}</p>}
    </form>
  );
};
