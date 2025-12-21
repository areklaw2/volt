export interface LogEntry {
  id: number;
  message: string;
  type: 'info' | 'connect' | 'disconnect' | 'message' | 'error';
  timestamp: string;
}
