import { SignIn } from '@clerk/react-router';

export default function SignInPage() {
  return (
    <div
      className="flex min-h-screen items-center justify-center"
      style={{
        background: 'linear-gradient(to bottom right, #3b82f6, #facc15)',
      }}
    >
      <SignIn
        appearance={{
          layout: {
            logoImageUrl: '/volt.svg',
            logoPlacement: 'inside',
          },
        }}
      />
    </div>
  );
}
