<script lang="ts">
  import { vaultStore } from '$lib/stores/vault';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { Lock } from 'lucide-svelte';
  import SearchBar from './SearchBar.svelte';
  import KeyTable from './KeyTable.svelte';
  import EmptyState from './EmptyState.svelte';
  import { uiStore } from '$lib/stores/ui';
  import { filteredKeys } from '$lib/stores/vault';
  import { isKeyModalOpen, isViewKeyModalOpen, isDeleteModalOpen } from '$lib/stores/ui';
  import KeyModal from '$lib/components/modals/KeyModal.svelte';
  import ViewKeyModal from '$lib/components/modals/ViewKeyModal.svelte';
  import DeleteModal from '$lib/components/modals/DeleteModal.svelte';
  import ThemeToggle from '$lib/components/ui/ThemeToggle.svelte';
</script>

<div class="vault-container container max-w-[1200px] mx-auto px-4 py-8 space-y-6">
  <!-- Header -->
  <header class="flex justify-between items-center">
    <div>
      <h1 class="text-3xl font-bold">API Keys</h1>
      <p class="text-muted-foreground">Manage your encrypted API keys</p>
    </div>
    <div class="flex gap-3 items-center">
      <ThemeToggle />
      <Button
        variant="primary"
        onclick={() => uiStore.openModal('key', null)}
      >
        + Add Key
      </Button>
      <Button
        variant="secondary"
        onclick={() => vaultStore.lock()}
      >
        <Lock class="w-4 h-4 mr-2" />
        Lock
      </Button>
    </div>
  </header>

  <!-- Search Bar -->
  <SearchBar />

  <!-- Keys List -->
  {#if $filteredKeys.length === 0}
    <EmptyState />
  {:else}
    <KeyTable />
  {/if}
</div>

<!-- Modals -->
<KeyModal open={$isKeyModalOpen} mode={$uiStore.keyModalMode} keyData={$uiStore.activeKeyId ? $filteredKeys.find(k => k.id === $uiStore.activeKeyId) : undefined} />
<ViewKeyModal open={$isViewKeyModalOpen} keyData={$filteredKeys.find(k => k.id === $uiStore.activeKeyId)!} />
<DeleteModal open={$isDeleteModalOpen} keyData={$filteredKeys.find(k => k.id === $uiStore.activeKeyId)!} />
