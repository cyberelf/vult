<script lang="ts">
  import { vaultStore, filteredKeys } from '$lib/stores/vault';
  import KeyActions from './KeyActions.svelte';
</script>

<div class="text-center py-16">
  {#if $vaultStore.searchQuery}
    <!-- No search results -->
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
        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
      />
    </svg>
    <h3 class="text-lg font-medium mb-2">No results found</h3>
    <p class="text-muted-foreground mb-6">
      No API keys match your search "{$vaultStore.searchQuery}"
    </p>
  {:else}
    <!-- No keys at all -->
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
  {/if}
</div>
