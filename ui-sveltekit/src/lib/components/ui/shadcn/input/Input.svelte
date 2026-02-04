<script lang="ts">
  import { cn } from '$lib/utils';

  type InputType = 'text' | 'password' | 'email' | 'url' | 'number' | 'tel';

  interface Props {
    type?: InputType;
    class?: string;
    placeholder?: string;
    value?: string;
    required?: boolean;
    minlength?: number;
    id?: string;
    name?: string;
    disabled?: boolean;
  }

  export let type: InputType = 'text';
  export let className: string = '';
  export let placeholder: string = '';
  export let value: string = '';
  export let required: boolean = false;
  export let minlength: number | undefined = undefined;
  export let id: string | undefined = undefined;
  export let name: string | undefined = undefined;
  export let disabled: boolean = false;
</script>

<input
  {type}
  {id}
  {name}
  {placeholder}
  {required}
  {minlength}
  {disabled}
  bind:value
  class={cn(
    'flex h-11 w-full rounded-md border border-border bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
    className
  )}
  on:input
  on:change
  on:focus
  on:blur
/>
