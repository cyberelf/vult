<script module lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  type Size = 'sm' | 'md' | 'lg' | 'icon';
  type Variant = 'default' | 'primary' | 'secondary' | 'danger' | 'outline' | 'ghost';

  interface Props extends HTMLButtonAttributes {
    variant?: Variant;
    size?: Size;
    class?: string;
  }

  const defaults: Omit<Props, 'class'> = {
    variant: 'default',
    size: 'md',
  };

  export let variant: Variant = defaults.variant;
  export let size: Size = defaults.size;
  export let className: string = '';

  const variants: Record<Variant, string> = {
    default: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
    primary: 'bg-primary text-primary-foreground hover:bg-primary/90',
    secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
    danger: 'bg-danger text-white hover:bg-danger/90',
    outline: 'border border-border bg-background hover:bg-accent hover:text-accent-foreground',
    ghost: 'hover:bg-accent hover:text-accent-foreground',
  };

  const sizes: Record<Size, string> = {
    sm: 'h-9 px-3 text-sm',
    md: 'h-11 px-5 py-2',
    lg: 'h-12 px-8 text-lg',
    icon: 'h-10 w-10',
  };
</script>

<button
  class={cn(
    'inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none',
    variants[variant],
    sizes[size],
    className
  )}
  on:click
  {...$$restProps}
>
  <slot />
</button>

<script lang="ts">
  import { cn } from '$lib/utils';
</script>
