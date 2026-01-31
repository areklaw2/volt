import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import { env } from '@/lib/env';
import {
  ClerkProvider,
  RedirectToSignIn,
  SignedIn,
  SignedOut,
} from '@clerk/react-router';
import { BrowserRouter, Routes, Route } from 'react-router';
import SignInPage from './components/sign-in';
import SignUpPage from './components/sign-up';
import App from './app';

const PUBLISHABLE_KEY = env.CLERK_PUBLISHABLE_KEY;
if (!PUBLISHABLE_KEY) {
  throw new Error('Missing Publishable Key');
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <ClerkProvider
        publishableKey={PUBLISHABLE_KEY}
        signInUrl="/sign-in"
        signUpUrl="/sign-up"
      >
        <Routes>
          <Route path="/sign-in/*" element={<SignInPage />} />
          <Route path="/sign-up/*" element={<SignUpPage />} />
          <Route
            path="/*"
            element={
              <>
                <SignedIn>
                  <App />
                </SignedIn>
                <SignedOut>
                  <RedirectToSignIn />
                </SignedOut>
              </>
            }
          />
        </Routes>
      </ClerkProvider>
    </BrowserRouter>
  </React.StrictMode>,
);
