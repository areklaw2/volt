import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { createUser } from '@/services/api';
import type { LocalIdentity } from '@/hooks/use-local-identity';

interface OnboardingProps {
  onComplete: (identity: LocalIdentity) => void;
}

export default function Onboarding({ onComplete }: OnboardingProps) {
  const [username, setUsername] = useState('');
  const [displayName, setDisplayName] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const canSubmit = username.trim().length > 0 && displayName.trim().length > 0 && !loading;

  const handleSubmit = async () => {
    if (!canSubmit) return;
    setLoading(true);
    setError(null);
    try {
      const id = crypto.randomUUID();
      await createUser(id, username.trim(), displayName.trim());
      onComplete({ id, username: username.trim(), displayName: displayName.trim() });
    } catch {
      setError('Could not create your account. Try again.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      className="flex min-h-screen items-center justify-center"
      style={{ background: 'linear-gradient(to bottom right, #3b82f6, #facc15)' }}
    >
      <div className="flex w-full max-w-sm flex-col gap-4 rounded-lg bg-background p-8 shadow-lg">
        <div className="flex flex-col gap-1 text-center">
          <img src="/volt.svg" alt="Volt" className="mx-auto h-10 w-10" />
          <h1 className="text-lg font-semibold">Welcome to Volt</h1>
          <p className="text-sm text-muted-foreground">Pick a username to get started.</p>
        </div>
        <Input
          placeholder="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleSubmit()}
        />
        <Input
          placeholder="Display name"
          value={displayName}
          onChange={(e) => setDisplayName(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleSubmit()}
        />
        {error && <p className="text-sm text-red-500">{error}</p>}
        <Button onClick={handleSubmit} disabled={!canSubmit}>
          {loading ? 'Creating...' : 'Continue'}
        </Button>
      </div>
    </div>
  );
}
