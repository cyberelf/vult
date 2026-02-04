<script lang="ts">
  import type { ApiKey } from '$lib/types';
  import { uiStore } from '$lib/stores/ui';
  import { Eye, Copy, Pencil, Trash2 } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { clipboardStore } from '$lib/stores/clipboard';
  import * as tauri from '$lib/services/tauri';

  interface Props {
    key: ApiKey;
    onToggleExpand?: () => void;
  }

  let { key, onToggleExpand }: Props = $props();
</script>

<div class="actions-cell flex justify-center items-center gap-1">
  <button
    class="p-2 rounded-md hover:bg-background-elevated hover:text-primary text-foreground-secondary transition-colors min-w-[36px] min-h-[36px] flex items-center justify-center"
    onclick={onToggleExpand}
    aria-label="View key details"
    title="View details"
  >
    <Eye class="w-4 h-4" />
  </button>

  <button
    class="p-2 rounded-md hover:bg-background-elevated hover:text-green-500 text-foreground-secondary transition-colors min-w-[36px] min-h-[36px] flex items-center justify-center"
    onclick={async () => {
      const keyValue = await tauri.copyApiKeyById(key.id);
      await clipboardStore.copy(keyValue, () => Promise.resolve());
    }}
    aria-label="Copy key"
    title="Copy to clipboard"
  >
    <Copy class="w-4 h-4" />
  </button>

  <button
    class="p-2 rounded-md hover:bg-background-elevated hover:text-primary text-foreground-secondary transition-colors min-w-[36px] min-h-[36px] flex items-center justify-center"
    onclick={() => uiStore.openModal('key', key.id)}
    aria-label="Edit key"
    title="Edit key"
  >
    <Pencil class="w-4 h-4" />
  </button>

  <button
    class="p-2 rounded-md hover:bg-background-elevated hover:text-danger text-foreground-secondary transition-colors min-w-[36px] min-h-[36px] flex items-center justify-center"
    onclick={() => uiStore.openModal('delete', key.id)}
    aria-label="Delete key"
    title="Delete key"
  >
    <Trash2 class="w-4 h-4" />
  </button>
</div>
