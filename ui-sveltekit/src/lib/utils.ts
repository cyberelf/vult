import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * Escapes HTML special characters to prevent XSS attacks.
 * @param input - The string to escape
 * @returns The escaped string
 */
export function escapeHtml(input: string): string {
  const map: Record<string, string> = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#x27;',
    '/': '&#x2F;',
  };
  return input.replace(/[&<>"'/]/g, (char) => map[char]);
}

const SECRET_ALPHABET =
  'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_';

/**
 * Generates a cryptographically secure, URL-safe secret.
 * The 64-character alphabet maps each random byte to exactly 6 bits without bias.
 */
export function generateSecret(length = 32): string {
  if (!Number.isInteger(length) || length <= 0) {
    throw new RangeError('Secret length must be a positive integer');
  }

  const randomBytes = new Uint8Array(length);
  crypto.getRandomValues(randomBytes);

  return Array.from(
    randomBytes,
    (byte) => SECRET_ALPHABET[byte & 63]
  ).join('');
}
