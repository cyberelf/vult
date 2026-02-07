<script lang="ts">
  import type { ApiKey } from '$lib/types';
  import { uiStore } from '$lib/stores/ui';
  import { Eye, Copy, Pencil, Trash2 } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { clipboardStore } from '$lib/stores/clipboard';
  import * as tauri from '$lib/services/tauri';
  import { fade } from 'svelte/transition';

  interface Props {
    key: ApiKey;
    onToggleExpand?: () => void;
  }

  let { key, onToggleExpand }: Props = $props();
</script>

<div class="actions-cell flex justify-center items-center gap-0.5 flex-wrap">
  <button
    class="p-1.5 rounded-md hover:bg-background-elevated hover:text-primary text-foreground-secondary transition-colors min-w-[32px] min-h-[32px] flex items-center justify-center"
    onclick={onToggleExpand}
    aria-label="View key details"
    title="View details"
    transition:fade={{ duration: 100 }}
  >
    <Eye class="w-4 h-4" />
  </button>

  <button
    class="p-1.5 rounded-md hover:bg-background-elevated hover:text-green-500 text-foreground-secondary transition-colors min-w-[32px] min-h-[32px] flex items-center justify-center"
    onclick={async () => {
      const keyValue = await tauri.copyApiKeyById(key.id);
      await clipboardStore.copy(keyValue, () => Promise.resolve());
    }}
    aria-label="Copy key"
    title="Copy to clipboard"
    transition:fade={{ duration: 100 }}
  >
    <Copy class="w-4 h-4" />
  </button>

  <button
    class="p-1.5 rounded-md hover:bg-background-elevated hover:text-primary text-foreground-secondary transition-colors min-w-[32px] min-h-[32px] flex items-center justify-center"
    onclick={() => uiStore.openModal('key', key.id)}
    aria-label="Edit key"
    title="Edit key"
    transition:fade={{ duration: 100 }}
  >
    <Pencil class="w-4 h-4" />
  </button>

  <button
    class="p-1.5 rounded-md hover:bg-background-elevated hover:text-danger text-foreground-secondary transition-colors min-w-[32px] min-h-[32px] flex items-center justify-center"
    onclick={() => uiStore.openModal('delete', key.id)}
    aria-label="Delete key"
    title="Delete key"
    transition:fade={{ duration: 100 }}
  >
    <Trash2 class="w-4 h-4" />
  </button>
</div>
