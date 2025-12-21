function getEnvVar(key: keyof ImportMetaEnv): string {
  const value = import.meta.env[key];
  return value || '';
}

// Export validated environment variables
export const env = {
  SOCKET_URL: getEnvVar('VITE_SOCKET_URL'),
  API_BASE_URL: getEnvVar('VITE_API_BASE_URL'),
  isDevelopment: import.meta.env.DEV,
  isProduction: import.meta.env.PROD,
} as const;
