export const env = {
  API_URL: import.meta.env.VITE_API_URL,
  WS_URL: import.meta.env.VITE_WS_URL,
  CLERK_PUBLISHABLE_KEY: import.meta.env.VITE_CLERK_PUBLISHABLE_KEY,
} as const;
