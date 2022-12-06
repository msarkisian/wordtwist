import { useState } from 'react';

// Generic hook, similar to `useState`, that syncs its state with localstorage
// defaults to 'initial' if the value is not currently in local storage
export const useLocalStorageState = <T,>(key: string, initial: T) => {
  let localStorageResult = !!window.localStorage.getItem(key);
  let value: T;

  if (!localStorageResult) {
    window.localStorage.setItem(key, JSON.stringify(initial));
    value = initial;
  } else {
    value = JSON.parse(window.localStorage.getItem(key)!);
  }

  const [item, setItem] = useState(value);

  const wrappedSetItem = (replacement: T) => {
    window.localStorage.setItem(key, JSON.stringify(replacement));
    setItem(replacement);
  };
  return [item, wrappedSetItem] as const;
};
