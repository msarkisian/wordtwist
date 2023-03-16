import React, { useState } from 'react';
import { Login } from './Login';

interface LoginProps {
  login: (username: string, password: string) => void;
  loginError: string | null;
}

export const LoginRegister: React.FC<LoginProps> = ({ login, loginError }) => {
  const [register, setRegister] = useState(false);
  return (
    <div className="flex justify-center my-10">
      <Login login={login} loginError={loginError} />
    </div>
  );
};
