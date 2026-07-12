import { useCallback, useState } from 'react';

export interface LocalIdentity {
  id: string;
  username: string;
  displayName: string;
}

const STORAGE_KEY = 'volt.identity';

function readIdentity(): LocalIdentity | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return raw ? (JSON.parse(raw) as LocalIdentity) : null;
  } catch {
    return null;
  }
}

export function useLocalIdentity() {
  const [identity, setIdentityState] = useState<LocalIdentity | null>(readIdentity);

  const setIdentity = useCallback((next: LocalIdentity) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
    setIdentityState(next);
  }, []);

  const clearIdentity = useCallback(() => {
    localStorage.removeItem(STORAGE_KEY);
    setIdentityState(null);
  }, []);

  return { identity, setIdentity, clearIdentity };
}
