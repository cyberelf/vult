<script lang="ts">
  import { vaultStore } from '$lib/stores/vault';
  import { Input } from '$lib/components/ui/shadcn/input';

  let searchInput = '';
  let debounceTimer: number;

  function handleSearchInput() {
    clearTimeout(debounceTimer);
    debounceTimer = window.setTimeout(() => {
      vaultStore.setSearchQuery(searchInput);
    }, 300);
  }
</script>

<div class="search-bar">
  <Input
    type="text"
    placeholder="Search keys by name, app, or description..."
    bind:value={searchInput}
    on:input={handleSearchInput}
    className="w-full max-w-[400px] min-w-[200px]"
  />
</div>
