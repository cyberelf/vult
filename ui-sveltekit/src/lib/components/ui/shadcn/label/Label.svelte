<script lang="ts">
  import { cn } from '$lib/utils';

  interface Props {
    for?: string;
    class?: string;
  }

  export let htmlFor: string | undefined = undefined;
  export let className: string = '';
</script>

<label
  for={htmlFor}
  class={cn(
    'text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70',
    className
  )}
>
  <slot />
</label>
