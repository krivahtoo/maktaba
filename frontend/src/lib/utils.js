import { dev } from '$app/environment';
import { clsx } from 'clsx';
import { auth } from './state/auth.svelte.js';
import { twMerge } from 'tailwind-merge';

/** @param {...import('clsx').ClassValue} inputs tailwind classes to merge. */
export function cn(...inputs) {
  return twMerge(clsx(inputs));
}

/**
 * @param {string} path
 * @param {RequestInit} options
 * @returns {Promise<Response>}
 */
export function cfetch(path, options = { credentials: 'same-origin' }) {
  const endpoint = dev ? 'http://0.0.0.0:3000/api' : '/api';
  const url = `${endpoint}${path}`;
  let headers = { ...options?.headers };
  if (dev && auth.token) {
    headers = { ...headers, Authorization: `Bearer ${auth.token}` };
  }
  options = { ...options, headers };
  return fetch(url, options);
}
