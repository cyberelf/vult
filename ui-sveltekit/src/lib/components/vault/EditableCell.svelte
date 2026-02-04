<script lang="ts">
  import { Check, X, Pencil } from 'lucide-svelte';
  import { tick } from 'svelte';

  interface Props {
    value?: string;
    onSave: (newValue: string) => Promise<void>;
    label?: string;
    placeholder?: string;
    class?: string;
  }

  let { 
    value = '', 
    onSave, 
    label = 'Edit', 
    placeholder = 'Empty', 
    class: className = '' 
  }: Props = $props();

  let isEditing = $state(false);
  let tempValue = $state(value);
  let inputRef: HTMLInputElement | undefined = $state();
  let isSaving = $state(false);

  async function startEditing(e?: Event) {
    if (isSaving) return;
    
    // Prevent event bubbling if triggered by click
    e?.stopPropagation();
    
    tempValue = value;
    isEditing = true;
    await tick();
    inputRef?.focus();
    inputRef?.select();
  }

  async function save() {
    if (isSaving) return;
    
    if (tempValue === value) {
      isEditing = false;
      return;
    }
    
    isSaving = true;
    try {
      await onSave(tempValue);
      isEditing = false;
    } catch (e) {
      console.error('Failed to save', e);
      // Keep editing state if failed
      inputRef?.focus();
    } finally {
      isSaving = false;
    }
  }

  function cancel() {
    if (isSaving) return;
    isEditing = false;
    tempValue = value;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault(); // Prevent form submission if any
      save();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      cancel();
    }
  }

  // Handle saving when focus leaves the input
  // We use a small timeout to allow button clicks (Save/Cancel) to register if they were the cause of blur
  // But simpler approach: use onmousedown on buttons to prevent blur.
  function handleBlur(e: FocusEvent) {
    // If the related target (what we are focusing) is one of our buttons, don't save yet.
    // The buttons will handle the action.
    // But `relatedTarget` logic is standard.
    // However, simplest is to just Save on blur.
    // If user clicked Cancel, they clicked Cancel.
    // But clicking Cancel moves focus. cancel() runs. Then handleBlur?
    // We'll rely on Save-on-blur as the primary "finish" action.
    // The "Cancel" button needs `onmousedown` preventDefault to allow its click handler to fire properly without blur race conditions?
    // Actually, usually "Escape" is for cancel. "Click outside" is Save.
    // Explicit "X" button should be Cancel.
    // If I click "X", blur fires. I save. Then X click fires?
    
    // Strategy: Focus-out = Save.
    // To Cancel, you MUST press Escape or use the X button (which we handle carefully).
    
    // For now, I'll rely on Enter/Escape mainly, but support Save-on-Blur.
    save();
  }
</script>

<div class="relative group w-full {className}">
  {#if isEditing}
    <div class="flex items-center w-full relative">
      <input
        bind:this={inputRef}
        bind:value={tempValue}
        class="w-full bg-background text-foreground px-2 py-1 text-sm border border-primary rounded outline-none ring-1 ring-primary/20 pr-14"
        {placeholder}
        disabled={isSaving}
        onkeydown={handleKeydown}
        onblur={handleBlur}
        onclick={(e) => e.stopPropagation()}
      />
      <!-- Actions overlay inside input -->
      <div class="absolute right-1 flex items-center gap-0.5 bg-background/90 rounded border border-border shadow-sm">
        <button 
            type="button"
            class="p-1 hover:bg-primary/20 hover:text-primary text-foreground-secondary rounded"
            onmousedown={(e) => e.preventDefault()} 
            onclick={save}
            title="Save (Enter)"
        >
            <Check class="w-3 h-3" />
        </button>
        <button 
            type="button"
            class="p-1 hover:bg-danger/20 hover:text-danger text-foreground-secondary rounded"
            onmousedown={(e) => e.preventDefault()}
            onclick={cancel}
            title="Cancel (Esc)"
        >
            <X class="w-3 h-3" />
        </button>
      </div>
    </div>
  {:else}
    <!-- View Mode -->
    <div 
        role="button" 
        tabindex="0"
        class="w-full min-h-[28px] px-2 py-1 -ml-2 rounded cursor-text border border-transparent hover:border-border-strong hover:bg-background-elevated/30 flex items-center transition-all group-hover:border-border-subtle"
        onclick={startEditing}
        onkeydown={(e) => e.key === 'Enter' && startEditing(e)}
        title="Click to edit"
        aria-label={`Edit ${label}`}
    >
      <span class="truncate block w-full whitespace-nowrap overflow-hidden text-ellipsis">
        {value || placeholder}
      </span>
      <span class="ml-2 text-foreground-secondary opacity-0 group-hover:opacity-50 transition-opacity shrink-0">
         <Pencil class="w-3 h-3" />
      </span>
    </div>
  {/if}
</div>