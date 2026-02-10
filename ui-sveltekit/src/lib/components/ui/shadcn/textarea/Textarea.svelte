<script lang="ts">
  import { cn } from '$lib/utils';

  interface Props {
    class?: string;
    placeholder?: string;
    value?: string;
    rows?: number | string;
    required?: boolean;
    id?: string;
    name?: string;
    disabled?: boolean;
  }

  export let className: string = '';
  export let placeholder: string = '';
  export let value: string = '';
  export let rows: number | string = 3;
  export let required: boolean = false;
  export let id: string | undefined = undefined;
  export let name: string | undefined = undefined;
  export let disabled: boolean = false;
</script>

<textarea
  {id}
  {name}
  {placeholder}
  {required}
  disabled={disabled}
  rows={typeof rows === 'string' ? parseInt(rows) : rows}
  bind:value
  class={cn(
    'flex min-h-[80px] w-full rounded-md border border-border bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
    className
  )}
  on:input
  on:change
  on:focus
  on:blur
></textarea>
