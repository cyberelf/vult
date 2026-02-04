<script lang="ts">
  import { vaultStore } from '$lib/stores/vault';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { Input } from '$lib/components/ui/shadcn/input';
  import { Label } from '$lib/components/ui/shadcn/label';

  let pin = $state('');
  let pinConfirm = $state('');
  let processing = $state(false);

  async function handleSubmit() {
    if (processing) return;

    // Client-side validation
    if (pin.length < 6) {
      return;
    }
    if (pin !== pinConfirm) {
      return;
    }

    processing = true;
    await vaultStore.setupVault(pin, pinConfirm);
    processing = false;
  }

  function getErrorMessage(): string | null {
    if (pin.length < 6 && pin.length > 0) {
      return 'PIN must be at least 6 characters';
    }
    if (pinConfirm && pin !== pinConfirm) {
      return 'PINs do not match';
    }
    return null;
  }

  import { error } from '$lib/stores/vault';
</script>

<div class="auth-container container max-w-[600px] mx-auto px-4 py-8">
  <div class="card bg-card rounded-lg p-8 shadow-lg">
    <div class="text-center mb-8">
      <h1 class="text-4xl font-bold mb-2">Vult</h1>
      <p class="text-muted-foreground">Secure API Key Vault</p>
      <p class="text-sm text-muted-foreground mt-4">
        Create a PIN to secure your vault. Remember this PIN - there is no recovery mechanism!
      </p>
    </div>

    <form on:submit|preventDefault={handleSubmit} class="space-y-6">
      <div class="space-y-2">
        <Label htmlFor="pin">PIN</Label>
        <Input
          id="pin"
          type="password"
          bind:value={pin}
          placeholder="Enter your PIN (min 6 characters)"
          minlength={6}
          required
          disabled={processing}
          autocomplete="new-password"
        />
      </div>

      <div class="space-y-2">
        <Label htmlFor="pin-confirm">Confirm PIN</Label>
        <Input
          id="pin-confirm"
          type="password"
          bind:value={pinConfirm}
          placeholder="Confirm your PIN"
          minlength={6}
          required
          disabled={processing}
          autocomplete="new-password"
        />
      </div>

      {#if getErrorMessage()}
        <p class="text-sm text-destructive">{getErrorMessage()}</p>
      {/if}

      {#if $error}
        <div class="p-3 bg-destructive/10 border border-destructive/20 rounded-md">
          <p class="text-sm text-destructive">{$error}</p>
        </div>
      {/if}

      <Button
        type="submit"
        variant="primary"
        class="w-full"
        disabled={processing || pin.length < 6 || pin !== pinConfirm}
      >
        {processing ? 'Creating Vault...' : 'Create Vault'}
      </Button>
    </form>
  </div>
</div>

<style>
  .card {
    background: var(--card);
  }
</style>
