<script lang="ts">
  import '../lib/css/app.css';
  import favicon from '$lib/assets/favicon.svg';
  import { onMount, onDestroy } from 'svelte';
  import { vaultStore } from '$lib/stores/vault';
  import Toast from '$lib/components/ui/toast/Toast.svelte';
  import { activityTracker } from '$lib/services/activity';

  let { children } = $props();

  // Initialize vault state on app load
  onMount(async () => {
    await vaultStore.initialize();
    // Initialize activity tracking after vault initialization
    await activityTracker.init();
  });

  // Cleanup activity tracking on app unmount
  onDestroy(async () => {
    await activityTracker.destroy();
  });
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Vult - Secure API Key Vault</title>
</svelte:head>

<div class="min-h-screen bg-background text-foreground">
  {@render children()}
  <Toast />
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
  }
</style>
