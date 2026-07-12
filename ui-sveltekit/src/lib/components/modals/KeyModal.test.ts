import { fireEvent, render, screen } from '@testing-library/svelte';
import { describe, expect, it } from 'vitest';
import KeyModal from './KeyModal.svelte';

describe('KeyModal secret controls', () => {
  it('generates a masked random secret in add mode', async () => {
    render(KeyModal, { open: true, mode: 'add' });

    const input = screen.getByLabelText('API Key') as HTMLInputElement;
    expect(input.type).toBe('password');

    await fireEvent.click(screen.getByRole('button', { name: 'Generate random secret' }));

    expect(input.value).toHaveLength(32);
    expect(input.value).toMatch(/^[A-Za-z0-9_-]+$/);
    expect(input.type).toBe('password');
  });

  it('toggles whether the API key is masked', async () => {
    render(KeyModal, { open: true, mode: 'add' });

    const input = screen.getByLabelText('API Key') as HTMLInputElement;
    await fireEvent.click(screen.getByRole('button', { name: 'Review API key' }));
    expect(input.type).toBe('text');

    await fireEvent.click(screen.getByRole('button', { name: 'Hide API key' }));
    expect(input.type).toBe('password');
  });

  it('does not offer random generation while editing an existing key', () => {
    render(KeyModal, {
      open: true,
      mode: 'edit',
    });

    expect(
      screen.queryByRole('button', { name: 'Generate random secret' })
    ).toBeNull();
    expect(screen.queryByRole('button', { name: 'Review API key' })).not.toBeNull();
  });
});
