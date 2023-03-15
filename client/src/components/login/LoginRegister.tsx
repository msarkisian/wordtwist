import React, { useState } from 'react';
import { Login } from './Login';

interface LoginProps {
  login: (username: string, password: string) => void;
}

export const LoginRegister: React.FC<LoginProps> = ({ login }) => {
  const [register, setRegister] = useState(false);
  return (
    <div className="flex justify-center my-10">
      <Login login={login} />
    </div>
  );
};
