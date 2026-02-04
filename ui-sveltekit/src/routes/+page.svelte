<script lang="ts">
  import SetupScreen from '$lib/components/auth/SetupScreen.svelte';
  import UnlockScreen from '$lib/components/auth/UnlockScreen.svelte';
  import VaultScreen from '$lib/components/vault/VaultScreen.svelte';
  import { currentScreen } from '$lib/stores/vault';
  import { fade } from 'svelte/transition';
</script>

<main class="screen min-h-screen flex items-center justify-center">
  {#if $currentScreen === 'setup'}
    <div transition:fade={{ duration: 200 }}>
      <SetupScreen />
    </div>
  {:else if $currentScreen === 'unlock'}
    <div transition:fade={{ duration: 200 }}>
      <UnlockScreen />
    </div>
  {:else if $currentScreen === 'vault'}
    <div transition:fade={{ duration: 200 }}>
      <VaultScreen />
    </div>
  {/if}
</main>

<style>
  .screen {
    background-color: var(--background);
  }
</style>
