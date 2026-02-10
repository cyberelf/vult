<script lang="ts">
  import { Button } from '$lib/components/ui/shadcn/button';
  import { Label } from '$lib/components/ui/shadcn/label';
  import { vaultStore } from '$lib/stores/vault';

  interface Props {
    open: boolean;
    onClose: () => void;
  }

  let { open = $bindable(false), onClose }: Props = $props();

  const biometricAvailable = $derived(
    $vaultStore.biometricAvailability === 'available'
  );

  async function handleToggleBiometric() {
    await vaultStore.toggleBiometric(!$vaultStore.biometricEnabled);
  }

  function getBiometricStatusText(): string {
    switch ($vaultStore.biometricAvailability) {
      case 'available':
        return 'Windows Hello is available';
      case 'not_configured':
        return 'Windows Hello is not configured';
      case 'device_not_present':
        return 'No biometric device found';
      case 'not_supported':
        return 'Not supported on this system';
      default:
        return 'Checking availability...';
    }
  }
</script>

  {#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4" onclick={onClose} role="button" tabindex="-1">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_interactive_supports_focus -->
    <div class="bg-card rounded-lg shadow-xl max-w-md w-full p-6 space-y-6" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true">
      <!-- Header -->
      <div class="space-y-2">
        <h2 class="text-2xl font-bold">Settings</h2>
        <p class="text-sm text-muted-foreground">Configure vault preferences</p>
      </div>

      <!-- Biometric Settings Section -->
      <div class="space-y-4">
        <div class="space-y-2">
          <h3 class="text-lg font-semibold">Windows Hello</h3>
          <p class="text-sm text-muted-foreground">
            {getBiometricStatusText()}
          </p>
        </div>

        {#if biometricAvailable}
          <div class="flex items-center justify-between p-4 bg-muted/50 rounded-lg">
            <div class="space-y-1 flex-1">
              <Label className="text-sm font-medium">Enable Windows Hello</Label>
              <p class="text-xs text-muted-foreground">
                Use biometric authentication to unlock the vault
              </p>
            </div>
            <button
              onclick={handleToggleBiometric}
              class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                $vaultStore.biometricEnabled
                  ? 'bg-primary'
                  : 'bg-muted-foreground/20'
              }`}
              role="switch"
              aria-checked={$vaultStore.biometricEnabled}
              aria-label="Toggle Windows Hello biometric authentication"
            >
              <span
                class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  $vaultStore.biometricEnabled ? 'translate-x-6' : 'translate-x-1'
                }`}
              ></span>
            </button>
          </div>

          {#if $vaultStore.biometricEnabled}
            <div class="p-3 bg-primary/10 border border-primary/20 rounded-md">
              <p class="text-xs text-primary">
                Windows Hello is enabled. You can unlock using biometric authentication or PIN.
              </p>
            </div>
          {/if}
        {:else}
          <div class="p-3 bg-muted/50 rounded-md">
            <p class="text-xs text-muted-foreground">
              {#if $vaultStore.biometricAvailability === 'not_configured'}
                Please configure Windows Hello in Windows Settings to use biometric unlock.
              {:else if $vaultStore.biometricAvailability === 'device_not_present'}
                No biometric device detected on this system.
              {:else}
                Windows Hello biometric unlock is only available on Windows 10 (1903+) and Windows 11.
              {/if}
            </p>
          </div>
        {/if}
      </div>

      <!-- Actions -->
      <div class="flex justify-end gap-3 pt-4 border-t">
        <Button variant="secondary" onclick={onClose}>
          Close
        </Button>
      </div>
    </div>
  </div>
{/if}
