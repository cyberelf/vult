<script lang="ts">
  import type { ApiKey } from '$lib/types';
  import { uiStore } from '$lib/stores/ui';
  import { Eye, Copy, Pencil, Trash2 } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { clipboardStore } from '$lib/stores/clipboard';
  import * as tauri from '$lib/services/tauri';

  interface Props {
    key: ApiKey;
  }

  let { key }: Props = $props();
</script>

<div class="actions-cell flex justify-center items-center gap-2">
  <button
    class="p-3 rounded hover:bg-primary/20 hover:text-primary text-muted-foreground transition-colors min-w-[44px] min-h-[44px] flex items-center justify-center"
    onclick={() => uiStore.openModal('view', key.id)}
    aria-label="View key"
  >
    <Eye class="w-4 h-4" />
  </button>

  <button
    class="p-3 rounded hover:bg-green-500/20 hover:text-green-500 text-muted-foreground transition-colors min-w-[44px] min-h-[44px] flex items-center justify-center"
    onclick={async () => {
      const keyValue = await tauri.copyApiKeyById(key.id);
      await clipboardStore.copy(keyValue, () => Promise.resolve());
    }}
    aria-label="Copy key"
  >
    <Copy class="w-4 h-4" />
  </button>

  <button
    class="p-3 rounded hover:bg-primary/20 hover:text-primary text-muted-foreground transition-colors min-w-[44px] min-h-[44px] flex items-center justify-center"
    onclick={() => uiStore.openModal('key', key.id)}
    aria-label="Edit key"
  >
    <Pencil class="w-4 h-4" />
  </button>

  <button
    class="p-3 rounded hover:bg-destructive/20 hover:text-destructive text-muted-foreground transition-colors min-w-[44px] min-h-[44px] flex items-center justify-center"
    onclick={() => uiStore.openModal('delete', key.id)}
    aria-label="Delete key"
  >
    <Trash2 class="w-4 h-4" />
  </button>
</div>
