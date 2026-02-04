<script lang="ts">
  import { uiStore } from '$lib/stores/ui';
  import { vaultStore } from '$lib/stores/vault';
  import * as tauri from '$lib/services/tauri';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { Input } from '$lib/components/ui/shadcn/input';
  import { Textarea } from '$lib/components/ui/shadcn/textarea';
  import { Label } from '$lib/components/ui/shadcn/label';
  import type { ApiKey } from '$lib/types';

  interface Props {
    open: boolean;
    mode: 'add' | 'edit';
    keyData?: ApiKey;
  }

  let { open, mode, keyData }: Props = $props();

  let appName = $state('');
  let keyName = $state('');
  let keyValue = $state('');
  let apiUrl = $state('');
  let description = $state('');
  let processing = $state(false);

  // Populate form when editing
  $effect(() => {
    if (mode === 'edit' && keyData) {
      appName = keyData.appName || '';
      keyName = keyData.keyName || '';
      keyValue = '';
      apiUrl = keyData.apiUrl || '';
      description = keyData.description || '';
    } else {
      // Reset form for add mode
      appName = '';
      keyName = '';
      keyValue = '';
      apiUrl = '';
      description = '';
    }
  });

  function getErrorMessage(): string | null {
    if (keyName.length === 0) return 'Key name is required';
    if (keyValue.length === 0) return 'API key is required';
    return null;
  }

  async function handleSubmit() {
    if (processing) return;

    const error = getErrorMessage();
    if (error) {
      uiStore.showToast(error, 'error');
      return;
    }

    processing = true;

    try {
      if (mode === 'add') {
        const newKey = await tauri.createApiKey({
          appName,
          keyName,
          keyValue,
          apiUrl: apiUrl || undefined,
          description: description || undefined,
        });
        vaultStore.addKey(newKey);
        uiStore.showToast('API key added successfully', 'success');
      } else if (mode === 'edit' && keyData) {
        const updatedKey = await tauri.updateApiKey({
          id: keyData.id,
          appName,
          keyName,
          keyValue,
          apiUrl: apiUrl || undefined,
          description: description || undefined,
        });
        vaultStore.updateKey(updatedKey);
        uiStore.showToast('API key updated successfully', 'success');
      }

      // Close modal and reset form
      uiStore.closeModal();
    } catch (err) {
      uiStore.showToast(
        err instanceof Error ? err.message : 'Failed to save key',
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
      class="relative bg-background-surface rounded-lg shadow-lg w-full max-w-[500px] max-h-[85vh] overflow-y-auto border border-border"
      role="dialog"
      aria-labelledby="modal-title"
    >
      <!-- Header -->
      <div class="flex justify-between items-center p-6 border-b border-border">
        <h2 id="modal-title" class="text-xl font-semibold">
          {mode === 'add' ? 'Add API Key' : 'Edit API Key'}
        </h2>
        <button
          class="p-2 hover:bg-muted rounded transition-colors"
          onclick={() => uiStore.closeModal()}
          aria-label="Close modal"
        >
          ✕
        </button>
      </div>

      <!-- Form -->
      <form onsubmit={handleSubmit} class="p-6 space-y-4">
        <div class="space-y-2">
          <Label htmlFor="app-name">App Name</Label>
          <Input
            id="app-name"
            type="text"
            bind:value={appName}
            placeholder="e.g., GitHub"
            disabled={processing}
          />
        </div>

        <div class="space-y-2">
          <Label htmlFor="key-name">Key Name</Label>
          <Input
            id="key-name"
            type="text"
            bind:value={keyName}
            placeholder="e.g., Personal Access Token"
            required
            disabled={processing}
          />
        </div>

        <div class="space-y-2">
          <Label htmlFor="key-value">API Key</Label>
          <Input
            id="key-value"
            type="password"
            bind:value={keyValue}
            placeholder="ghp_xxxxxxxxxxxx"
            required
            disabled={processing}
          />
        </div>

        <div class="space-y-2">
          <Label htmlFor="api-url">API URL (Optional)</Label>
          <Input
            id="api-url"
            type="url"
            bind:value={apiUrl}
            placeholder="https://api.github.com"
            disabled={processing}
          />
        </div>

        <div class="space-y-2">
          <Label htmlFor="description">Description (Optional)</Label>
          <Textarea
            id="description"
            bind:value={description}
            placeholder="Used for GitHub API access"
            rows="3"
            disabled={processing}
          />
        </div>

        <!-- Error Display -->
        {#if getErrorMessage()}
          <p class="text-sm text-destructive">{getErrorMessage()}</p>
        {/if}

        <!-- Actions -->
        <div class="flex justify-end gap-3 pt-4 border-t border-border">
          <Button
            type="button"
            variant="secondary"
            onclick={() => uiStore.closeModal()}
            disabled={processing}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            variant="primary"
            disabled={processing}
          >
            {processing ? 'Saving...' : 'Save'}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
