<script lang="ts">
  import { themeStore } from '$lib/stores/theme';
  import { Sun, Moon } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/shadcn/button';

  let theme = $derived($themeStore);
</script>

<Button
  variant="ghost"
  size="icon"
  onclick={() => themeStore.toggle()}
  title={theme === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}
  aria-label="Toggle theme"
  class="transition-transform hover:scale-110"
>
  {#if theme === 'light'}
    <Sun class="w-5 h-5" />
  {:else}
    <Moon class="w-5 h-5" />
  {/if}
</Button>
