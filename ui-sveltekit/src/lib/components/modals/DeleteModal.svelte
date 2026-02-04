<script lang="ts">
  import { uiStore } from '$lib/stores/ui';
  import { vaultStore } from '$lib/stores/vault';
  import * as tauri from '$lib/services/tauri';
  import { Button } from '$lib/components/ui/shadcn/button';
  import type { ApiKey } from '$lib/types';

  interface Props {
    open: boolean;
    keyData: ApiKey;
  }

  let { open, keyData }: Props = $props();
  let processing = $state(false);

  async function handleConfirm() {
    if (processing) return;

    processing = true;

    try {
      await tauri.deleteApiKey(keyData.id);
      vaultStore.removeKey(keyData.id);
      uiStore.showToast('API key deleted successfully', 'success');
      uiStore.closeModal();
    } catch (err) {
      uiStore.showToast(
        err instanceof Error ? err.message : 'Failed to delete key',
        'error'
      );
    } finally {
      processing = false;
    }
  }
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <!-- Backdrop -->
    <div
      class="absolute inset-0 bg-black/80"
      onclick={() => uiStore.closeModal()}
      role="presentation"
    ></div>

    <!-- Modal -->
    <div
      class="relative bg-background-surface rounded-lg shadow-lg w-full max-w-[400px] p-6 border border-border"
      role="alertdialog"
      aria-labelledby="delete-modal-title"
      aria-describedby="delete-modal-description"
      tabindex="-1"
      onkeydown={(e) => e.key === 'Escape' && uiStore.closeModal()}
    >
      <!-- Icon -->
      <div class="flex justify-center mb-4">
        <div
          class="w-12 h-12 rounded-full bg-destructive/10 flex items-center justify-center"
        >
          <svg
            class="w-6 h-6 text-destructive"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.932-3L13.062 8.24c-.378-1.075-1.38-1.932-2.502-1.932H4.062c-1.122 0-2.124.857-2.502 1.932L2.938 16"
            />
          </svg>
        </div>
      </div>

      <!-- Title -->
      <h2
        id="delete-modal-title"
        class="text-xl font-semibold text-center mb-2"
      >
        Delete API Key
      </h2>

      <!-- Description -->
      <p
        id="delete-modal-description"
        class="text-center text-muted-foreground mb-6"
      >
        Are you sure you want to delete "<strong>{keyData.keyName}</strong
        >"? This action cannot be undone.
      </p>

      <!-- Key details -->
      <div class="bg-muted/50 rounded-lg p-4 mb-6 space-y-2 text-sm">
        <p><span class="font-medium">App:</span> {keyData.appName}</p>
        <p>
          <span class="font-medium">Key:</span>{' '}
          {keyData.keyName}
        </p>
      </div>

      <!-- Actions -->
      <div class="flex justify-center gap-3">
        <Button
          variant="secondary"
          onclick={() => uiStore.closeModal()}
          disabled={processing}
        >
          Cancel
        </Button>
        <Button
          variant="danger"
          onclick={handleConfirm}
          disabled={processing}
        >
          {processing ? 'Deleting...' : 'Delete'}
        </Button>
      </div>
    </div>
  </div>
{/if}
