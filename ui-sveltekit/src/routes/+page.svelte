<script lang="ts">
  import SetupScreen from '$lib/components/auth/SetupScreen.svelte';
  import UnlockScreen from '$lib/components/auth/UnlockScreen.svelte';
  import VaultScreen from '$lib/components/vault/VaultScreen.svelte';
  import { currentScreen } from '$lib/stores/vault';
</script>

<main class="screen min-h-screen flex items-center justify-center">
  {#if $currentScreen === 'setup'}
    <SetupScreen />
  {:else if $currentScreen === 'unlock'}
    <UnlockScreen />
  {:else if $currentScreen === 'vault'}
    <VaultScreen />
  {/if}
</main>

<style>
  .screen {
    background-color: var(--background);
  }
</style>
