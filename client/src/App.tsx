import { UserButton } from '@clerk/react-router';

function App() {
  return (
    <div>
      <header className="p-4 flex justify-end">
        <UserButton />
      </header>
      <main className="p-4">
        <h1>Welcome to Volt Chat</h1>
        {/* Chat UI will go here */}
      </main>
    </div>
  );
}

export default App;
