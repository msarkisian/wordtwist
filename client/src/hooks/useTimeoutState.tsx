import { useRef, useState } from 'react';

// `useState` equilivant that reverts state back to `null` after timeout expires
export const useTimeoutState = <T,>(timeout: number) => {
  const [state, setState] = useState<T | null>(null);
  const timeoutRef = useRef<number | null>(null);

  const wrappedSetState = (newState: T) => {
    setState(newState);
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
    setTimeout(() => {
      setState(null);
    }, timeout);
  };
  return [state, wrappedSetState] as const;
};
