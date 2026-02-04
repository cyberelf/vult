<script lang="ts">
  import { filteredKeys } from '$lib/stores/vault';
  import KeyActions from './KeyActions.svelte';
</script>

<div class="text-center py-16">
  <svg
    class="mx-auto h-16 w-16 text-muted-foreground mb-4"
    fill="none"
    viewBox="0 0 24 24"
    stroke="currentColor"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
      d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"
    />
  </svg>
  <h3 class="text-lg font-medium mb-2">No API keys yet</h3>
  <p class="text-muted-foreground mb-6">
    Add your first API key to get started
  </p>
</div>
