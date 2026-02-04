<script lang="ts">
  import { uiStore } from '$lib/stores/ui';
  import { fade } from 'svelte/transition';
  import { CircleCheck, AlertCircle, Info } from 'lucide-svelte';
  import { onMount } from 'svelte';

  let visible = false;
  let toastMessage = '';
  let toastType: 'success' | 'error' | 'info' = 'info';

  onMount(() => {
    const unsub = uiStore.subscribe((state) => {
      if (state.toast) {
        toastMessage = state.toast;
        toastType = state.toastType;
        visible = true;
      } else {
        visible = false;
      }
    });
    return unsub;
  });

  const iconMap = {
    success: CircleCheck,
    error: AlertCircle,
    info: Info,
  };
</script>

{#if visible}
  <div
    class="fixed bottom-4 right-4 z-[100] flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg min-w-[300px] max-w-[500px] text-white"
    class:bg-success={toastType === 'success'}
    class:bg-destructive={toastType === 'error'}
    class:bg-primary={toastType === 'info'}
    transition:fade={{ duration: 200 }}
  >
    <svelte:component this={iconMap[toastType]} class="w-5 h-5 flex-shrink-0" />
    <span class="flex-1 text-sm font-medium">{toastMessage}</span>
  </div>
{/if}
