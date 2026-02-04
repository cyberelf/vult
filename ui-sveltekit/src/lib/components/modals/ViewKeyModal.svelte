<script lang="ts">
  import { uiStore } from '$lib/stores/ui';
  import { clipboardStore, copiedKeyDisplay, hasCopiedKey } from '$lib/stores/clipboard';
  import * as tauri from '$lib/services/tauri';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { Eye, EyeOff, Copy, Pencil, Trash2 } from 'lucide-svelte';
  import type { ApiKey } from '$lib/types';

  interface Props {
    open: boolean;
    keyData: ApiKey;
  }

  let { open, keyData }: Props = $props();
  let showKey = $state(false);

  async function handleCopy() {
    const keyValue = await tauri.copyApiKeyById(keyData.id);
    await clipboardStore.copy(keyValue, () => Promise.resolve());
  }

  function handleEdit() {
    uiStore.closeModal();
    uiStore.openModal('key', keyData.id);
  }

  function handleDelete() {
    uiStore.closeModal();
    uiStore.openModal('delete', keyData.id);
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
      class="relative bg-card rounded-lg shadow-lg w-full max-w-[500px"
      role="dialog"
      aria-labelledby="view-modal-title"
      tabindex="-1"
      onkeydown={(e) => e.key === 'Escape' && uiStore.closeModal()}
    >
      <!-- Header -->
      <div class="flex justify-between items-center p-6 border-b border-border">
        <h2 id="view-modal-title" class="text-xl font-semibold">
          {keyData.keyName}
        </h2>
        <button
          class="p-2 hover:bg-muted rounded transition-colors"
          onclick={() => uiStore.closeModal()}
          aria-label="Close modal"
        >
          ✕
        </button>
      </div>

      <!-- Content -->
      <div class="p-6 space-y-4">
        <div>
          <p class="text-sm text-muted-foreground mb-1">App Name</p>
          <p class="font-medium">{keyData.appName || '-'}</p>
        </div>

        <div>
          <p class="text-sm text-muted-foreground mb-1">API Key</p>
          <div class="flex items-center gap-2">
            <code class="flex-1 font-mono text-sm bg-muted px-3 py-2 rounded flex items-center">
              {showKey
                ? keyData.keyValue
                : '•'.repeat(Math.min(keyData.keyValue?.length || 20, 20))}
            </code>
            <button
              class="p-2 hover:bg-muted rounded transition-colors"
              onclick={() => showKey = !showKey}
              aria-label={showKey ? 'Hide key' : 'Show key'}
            >
              {#if showKey}
                <EyeOff class="w-4 h-4" />
              {:else}
                <Eye class="w-4 h-4" />
              {/if}
            </button>
          </div>
        </div>

        {#if keyData.apiUrl}
          <div>
            <p class="text-sm text-muted-foreground mb-1">API URL</p>
            <p class="font-mono text-sm break-all">{keyData.apiUrl}</p>
          </div>
        {/if}

        {#if keyData.description}
          <div>
            <p class="text-sm text-muted-foreground mb-1">Description</p>
            <p class="text-sm">{keyData.description}</p>
          </div>
        {/if}

        <div class="text-xs text-muted-foreground">
          <p>Created: {new Date(keyData.createdAt).toLocaleString()}</p>
          <p>Updated: {new Date(keyData.updatedAt).toLocaleString()}</p>
        </div>

        <!-- Actions -->
        <div class="flex flex-wrap gap-2 pt-4 border-t border-border">
          <Button
            variant="secondary"
            onclick={handleCopy}
          >
            <Copy class="w-4 h-4 mr-2" />
            Copy
          </Button>

          {#if $hasCopiedKey}
            <p class="text-sm text-green-500 flex items-center">
              ✓ Copied! Clears in 45s
            </p>
          {/if}

          <Button
            variant="primary"
            onclick={handleEdit}
          >
            <Pencil class="w-4 h-4 mr-2" />
            Edit
          </Button>

          <Button
            variant="danger"
            onclick={handleDelete}
          >
            <Trash2 class="w-4 h-4 mr-2" />
            Delete
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}
