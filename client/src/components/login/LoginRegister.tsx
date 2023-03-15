import React, { useState } from 'react';
import { Login } from './Login';

interface LoginProps {}

export const LoginRegister: React.FC<LoginProps> = ({}) => {
  const [register, setRegister] = useState(false);
  return (
    <div className="flex justify-center my-10">
      <Login />
    </div>
  );
};
