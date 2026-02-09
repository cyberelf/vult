<script lang="ts">
  import { vaultStore } from '$lib/stores/vault';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { Input } from '$lib/components/ui/shadcn/input';
  import { Label } from '$lib/components/ui/shadcn/label';
  import { error, isLoading } from '$lib/stores/vault';

  let pin = $state('');
  let processing = $state(false);
  let showPinInput = $state(false); // Start with Windows Hello if available

  // Determine if Windows Hello should be shown as default
  const biometricAvailable = $derived(
    $vaultStore.biometricAvailability === 'available' && 
    $vaultStore.biometricEnabled
  );
  
  const biometricReady = $derived(
    biometricAvailable && $vaultStore.biometricStorageEnabled
  );
  
  // Show Windows Hello by default if ready, otherwise show PIN
  $effect(() => {
    if (!biometricAvailable || !$vaultStore.biometricStorageEnabled) {
      showPinInput = true;
    } else {
      showPinInput = false;
    }
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();
    if (processing) return;
    processing = true;
    await vaultStore.unlock(pin);
    processing = false;
  }

  async function handleBiometricUnlock() {
    if (processing) return; // Prevent double-click
    processing = true;
    try {
      await vaultStore.unlockWithBiometric();
    } catch (err) {
      // Biometric failed - show PIN input for fallback
      showPinInput = true;
    } finally {
      processing = false; // Always reset processing state
    }
  }

  function showPinFallback() {
    showPinInput = true;
  }
</script>

<div class="auth-container container max-w-[600px] mx-auto px-4 py-8">
  <div class="card bg-card rounded-lg p-8 shadow-lg">
    <div class="text-center mb-8">
      <h1 class="text-4xl font-bold mb-2">Vult</h1>
      <p class="text-muted-foreground">Secure API Key Vault</p>
      <p class="text-sm text-muted-foreground mt-4">
        {#if biometricReady && !showPinInput}
          Click below to unlock with Windows Hello
        {:else if biometricAvailable && !$vaultStore.biometricStorageEnabled}
          Unlock with PIN first to enable Windows Hello
        {:else}
          Enter your PIN to unlock the vault
        {/if}
      </p>
    </div>

    {#if $error}
      <div class="p-3 bg-destructive/10 border border-destructive/20 rounded-md mb-6">
        <p class="text-sm text-destructive">{$error}</p>
      </div>
    {/if}

    {#if biometricReady && !showPinInput}
      <!-- Windows Hello unlock (default when available) -->
      <div class="space-y-4">
        <Button
          onclick={handleBiometricUnlock}
          variant="primary"
          class="w-full py-8 border-2 border-primary/20 flex items-center justify-center"
          disabled={processing || $isLoading}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6 mr-2"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path fill-rule="evenodd" d="M6.267 3.455a3.066 3.066 0 001.745-.723 3.066 3.066 0 013.976 0 3.066 3.066 0 001.745.723 3.066 3.066 0 012.812 2.812c.051.643.304 1.254.723 1.745a3.066 3.066 0 010 3.976 3.066 3.066 0 00-.723 1.745 3.066 3.066 0 01-2.812 2.812 3.066 3.066 0 00-1.745.723 3.066 3.066 0 01-3.976 0 3.066 3.066 0 00-1.745-.723 3.066 3.066 0 01-2.812-2.812 3.066 3.066 0 00-.723-1.745 3.066 3.066 0 010-3.976 3.066 3.066 0 00.723-1.745 3.066 3.066 0 012.812-2.812zm7.44 5.252a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
          <span>{processing || $isLoading ? 'Unlocking...' : 'Unlock with Windows Hello'}</span>
        </Button>

        <Button
          onclick={showPinFallback}
          variant="secondary"
          class="w-full"
          disabled={processing || $isLoading}
        >
          Use PIN instead
        </Button>
      </div>
    {:else}
      <!-- PIN unlock form -->
      <form onsubmit={handleSubmit} class="space-y-6">
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

        <Button
          type="submit"
          variant="primary"
          class="w-full py-8 border-2 border-primary/20 flex items-center justify-center"
          disabled={processing || $isLoading || pin.length < 6}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6 mr-2"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd" />
          </svg>
          <span>{processing || $isLoading ? 'Unlocking...' : 'Unlock'}</span>
        </Button>

        {#if biometricReady}
          <Button
            onclick={() => showPinInput = false}
            type="button"
            variant="secondary"
            class="w-full"
            disabled={processing || $isLoading}
          >
            Use Windows Hello instead
          </Button>
        {/if}
      </form>
    {/if}

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
