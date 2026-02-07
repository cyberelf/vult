<script lang="ts">
  import SetupScreen from '$lib/components/auth/SetupScreen.svelte';
  import UnlockScreen from '$lib/components/auth/UnlockScreen.svelte';
  import VaultScreen from '$lib/components/vault/VaultScreen.svelte';
  import { currentScreen, isLoading } from '$lib/stores/vault';
  import { fade, fly } from 'svelte/transition';
  import { onMount } from 'svelte';
</script>

<main class="screen min-h-screen flex items-center justify-center">
  <!-- Keep background consistent during transitions -->
  <div class="fixed inset-0 bg-background"></div>

  <div class="relative z-10 w-full flex items-center justify-center min-h-screen">
    {#if $currentScreen === 'setup'}
      <div
        transition:fade={{ duration: 200 }}
        class="w-full max-w-md"
      >
        <SetupScreen />
      </div>
    {:else if $currentScreen === 'unlock'}
      <div
        transition:fade={{ duration: 200 }}
        class="w-full max-w-md"
      >
        <UnlockScreen />
      </div>
    {:else if $currentScreen === 'vault'}
      <div
        transition:fade={{ duration: 200 }}
        class="w-full max-w-[1200px]"
      >
        <VaultScreen />
      </div>
    {/if}

    <!-- Loading overlay -->
    {#if $isLoading}
      <div
        class="fixed inset-0 bg-background/90 backdrop-blur-sm z-50 flex items-center justify-center"
        transition:fade={{ duration: 100 }}
      >
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-primary border-t-transparent"></div>
      </div>
    {/if}
  </div>
</main>

<style>
  .screen {
    background-color: var(--background);
    overflow: hidden; /* Prevent scrollbars during transitions */
  }

  /* Ensure smooth transitions for all screen components */
  div[transition\:fade] {
    will-change: opacity;
  }
</style>
