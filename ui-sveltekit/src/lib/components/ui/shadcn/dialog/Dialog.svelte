<script lang="ts">
  import { cn } from '$lib/utils';
  import { createModal, dismissable, focusTrap } from 'bits-ui';
  import { X } from 'lucide-svelte';

  interface Props {
    open?: boolean;
    title?: string;
    description?: string;
    class?: string;
    contentClass?: string;
  }

  export let open: boolean = false;
  export let title: string = '';
  export let description: string = '';
  export let className: string = '';
  export let contentClass: string = '';

  const {
    elements: { Content, Overlay, Title, Description, Portal, Close },
    states: { open },
    helpers: { handleOpenChange },
  } = createModal({
    closeOnOutsideClick: true,
    closeOnEscape: true,
    role: 'dialog',
  });

  $: handleOpenChange(open);
</script>

<Portal>
  <Overlay
    class="fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0"
  />
  <Content
    class={cn(
      'fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg',
      contentClass
    )}
    use:dismissable
    use:focusTrap
  >
    <slot name="header">
      {#if title}
        <Title class="text-lg font-semibold leading-none tracking-tight">
          {title}
        </Title>
      {/if}
      {#if description}
        <Description class="text-sm text-muted-foreground">
          {description}
        </Description>
      {/if}
    </slot>

    <slot />

    <Close
      class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground min-w-[44px] min-h-[44px] flex items-center justify-center"
      aria-label="Close modal"
    >
      <X class="h-4 w-4" />
    </Close>
  </Content>
</Portal>
