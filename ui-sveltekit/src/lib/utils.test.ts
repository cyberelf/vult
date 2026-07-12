/**
 * Tests for utility functions
 */

import { describe, it, expect } from 'vitest';
import { escapeHtml, cn, generateSecret } from '$lib/utils';

describe('escapeHtml', () => {
  it('should escape HTML entities', () => {
    const input = '<script>alert("xss")</script>';
    const output = escapeHtml(input);

    expect(output).toContain('&lt;');
    expect(output).toContain('&gt;');
    expect(output).not.toContain('<script>');
  });

  it('should escape quotes', () => {
    const input = 'Test "quoted" and \'single\'';
    const output = escapeHtml(input);

    expect(output).toContain('&quot;');
    expect(output).toContain('&#x27;');
  });

  it('should escape ampersands', () => {
    const input = 'Tom & Jerry';
    const output = escapeHtml(input);

    expect(output).toContain('&amp;');
  });

  it('should escape forward slashes', () => {
    const input = 'path/to/file';
    const output = escapeHtml(input);

    expect(output).toContain('&#x2F;');
  });

  it('should handle empty strings', () => {
    expect(escapeHtml('')).toBe('');
  });

  it('should handle strings without special characters', () => {
    expect(escapeHtml('Hello World')).toBe('Hello World');
  });
});

describe('cn (className utility)', () => {
  it('should merge class names correctly', () => {
    expect(cn('foo', 'bar')).toBe('foo bar');
  });

  it('should handle conditional classes', () => {
    expect(cn('foo', false && 'bar', 'baz')).toBe('foo baz');
  });

  it('should handle undefined and null', () => {
    expect(cn('foo', undefined, null, 'bar')).toBe('foo bar');
  });

  it('should handle empty strings', () => {
    expect(cn('foo', '', 'bar')).toBe('foo bar');
  });

  it('should handle Tailwind class conflicts (later wins)', () => {
    expect(cn('p-4', 'p-2')).toBe('p-2');
  });
});

describe('generateSecret', () => {
  it('should generate a 32-character URL-safe secret by default', () => {
    const secret = generateSecret();

    expect(secret).toHaveLength(32);
    expect(secret).toMatch(/^[A-Za-z0-9_-]+$/);
  });

  it('should support a custom secret length', () => {
    expect(generateSecret(48)).toHaveLength(48);
  });

  it('should reject invalid secret lengths', () => {
    expect(() => generateSecret(0)).toThrow(RangeError);
    expect(() => generateSecret(1.5)).toThrow(RangeError);
  });
});
