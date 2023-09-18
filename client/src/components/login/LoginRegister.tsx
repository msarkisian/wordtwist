import React, { useState } from 'react';
import { Login } from './Login';

interface LoginProps {
  showRegister: boolean;
  login: (username: string, password: string) => void;
  loginError: string | null;
  register: (email: string, username: string, password: string) => void;
}

export const LoginRegister: React.FC<LoginProps> = ({
  login,
  loginError,
  showRegister,
  register,
}) => {
  return (
    <div className="flex justify-center my-10">
      <Login
        login={login}
        loginError={loginError}
        showRegister={showRegister}
        register={register}
      />
    </div>
  );
};
