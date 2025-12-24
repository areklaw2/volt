import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useChatContext } from '@/contexts/ChatContext';

export function LoginPage() {
  const [username, setUsername] = useState('');
  const { state, setUserId } = useChatContext();
  const navigate = useNavigate();

  // Redirect to chat if already logged in
  useEffect(() => {
    const savedUserId = localStorage.getItem('currentUserId');
    if (savedUserId || state.currentUserId) {
      navigate('/chat');
    }
  }, [state.currentUserId, navigate]);

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    if (username.trim()) {
      setUserId(username.trim());
      navigate('/chat');
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-background">
      <div className="w-full max-w-md p-8 space-y-6">
        <div className="text-center space-y-2">
          <h1 className="text-3xl font-bold">Welcome to Volt Chat</h1>
          <p className="text-muted-foreground">
            Enter your username to start chatting
          </p>
        </div>
        <form onSubmit={handleLogin} className="space-y-4">
          <Input
            type="text"
            placeholder="Username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            className="w-full"
            autoFocus
          />
          <Button type="submit" className="w-full" disabled={!username.trim()}>
            Continue
          </Button>
        </form>
      </div>
    </div>
  );
}
