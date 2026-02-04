<script lang="ts">
  import { vaultStore } from '$lib/stores/vault';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { Input } from '$lib/components/ui/shadcn/input';
  import { Label } from '$lib/components/ui/shadcn/label';

  let pin = $state('');
  let processing = $state(false);

  async function handleSubmit() {
    if (processing) return;
    processing = true;
    await vaultStore.unlock(pin);
    processing = false;
  }

  import { error, isLoading } from '$lib/stores/vault';
</script>

<div class="auth-container container max-w-[600px] mx-auto px-4 py-8">
  <div class="card bg-card rounded-lg p-8 shadow-lg">
    <div class="text-center mb-8">
      <h1 class="text-4xl font-bold mb-2">Vult</h1>
      <p class="text-muted-foreground">Secure API Key Vault</p>
      <p class="text-sm text-muted-foreground mt-4">
        Enter your PIN to unlock the vault
      </p>
    </div>

    <form on:submit|preventDefault={handleSubmit} class="space-y-6">
      <div class="space-y-2">
        <Label htmlFor="unlock-pin">PIN</Label>
        <Input
          id="unlock-pin"
          type="password"
          bind:value={pin}
          placeholder="Enter your PIN"
          required
          disabled={processing || $isLoading}
          autocomplete="current-password"
          autofocus
        />
      </div>

      {#if $error}
        <div class="p-3 bg-destructive/10 border border-destructive/20 rounded-md">
          <p class="text-sm text-destructive">{$error}</p>
        </div>
      {/if}

      <Button
        type="submit"
        variant="primary"
        class="w-full"
        disabled={processing || $isLoading || pin.length < 6}
      >
        {processing || $isLoading ? 'Unlocking...' : 'Unlock'}
      </Button>
    </form>

    <div class="mt-6 text-center">
      <p class="text-xs text-muted-foreground">
        Vault will auto-lock after 5 minutes of inactivity
      </p>
    </div>
  </div>
</div>

<style>
  .card {
    background: var(--card);
  }
</style>
