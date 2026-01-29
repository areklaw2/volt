import { SignUp } from '@clerk/react-router';

export default function SignUpPage() {
  return (
    <div
      className="flex min-h-screen items-center justify-center"
      style={{
        background: 'linear-gradient(to bottom right, #3b82f6, #facc15)',
      }}
    >
      <SignUp
        forceRedirectUrl="/"
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
