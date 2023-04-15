/* eslint-disable no-console */
/*
 * This module implement logging functionality in the frontend.
 *
 * Logging is only enabled when the frontend is not built for prod.
 */

export const enum LogLevel {
  ERROR = 1,
  WARN = 2,
  INFO = 3,
  DEBUG = 4,
  TRACE = 5,
}

export function log(level: LogLevel, ...data: unknown[]): void {
  if (import.meta.env.PROD) {
    return;
  }
  switch (level) {
    case LogLevel.ERROR:
      console.error(...data);
      break;
    case LogLevel.WARN:
      console.warn(...data);
      break;
    case LogLevel.INFO:
      console.info(...data);
      break;
    case LogLevel.DEBUG:
      console.debug(...data);
      break;
    case LogLevel.TRACE:
      console.trace(...data);
      break;
  }
}

export function error(...data: unknown[]): void {
  log(LogLevel.ERROR, ...data);
}

export function warn(...data: unknown[]): void {
  log(LogLevel.WARN, ...data);
}

export function info(...data: unknown[]): void {
  log(LogLevel.INFO, ...data);
}

export function debug(...data: unknown[]): void {
  log(LogLevel.DEBUG, ...data);
}

export function trace(...data: unknown[]): void {
  log(LogLevel.TRACE, ...data);
}
