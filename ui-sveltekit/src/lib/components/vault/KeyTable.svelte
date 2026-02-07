<script lang="ts">
  import { filteredKeys, vaultStore } from '$lib/stores/vault';
  import { uiStore } from '$lib/stores/ui';
  import KeyActions from './KeyActions.svelte';
  import EditableCell from './EditableCell.svelte';
  import * as tauri from '$lib/services/tauri';

  let expandedKeyId = $state<number | null>(null);
  let decryptedValues = $state<Record<number, string>>({});

  function toggleExpand(id: number) {
      if (expandedKeyId === id) {
          expandedKeyId = null;
      } else {
          expandedKeyId = id;
      }
  }

  async function toggleReveal(id: number) {
      if (decryptedValues[id]) {
          const newValues = { ...decryptedValues };
          delete newValues[id];
          decryptedValues = newValues;
      } else {
          try {
             // We need a way to get the RAW value to display. 
             // copyApiKeyById gets it for clipboard.
             // We'll need to reuse that or add a new method?
             // Actually copyApiKeyById returns the string!
             const val = await tauri.copyApiKeyById(id);
             // BUT copyApiKeyById ALSO copies to clipboard. That might be a side effect we don't want just for revealing?
             // Ideally we have `getDecryptedKey(id)`.
             // For now, let's use it as is, or maybe the user is okay with it also copying?
             // "Reveal" implies seeing it.
             decryptedValues = { ...decryptedValues, [id]: val };
          } catch(e) {
              console.error(e);
          }
      }
  }
</script>

<div class="@container flex flex-col gap-4">
  <!-- Desktop Table View (>512px) -->
  <div class="hidden @lg:block rounded-md border border-border bg-background-surface overflow-hidden shadow-sm">
    <table class="w-full text-left">
      <thead class="bg-background-subtle border-b border-border">
        <tr>
          <th class="p-6 text-sm font-semibold text-foreground-secondary uppercase tracking-wider">
            Key Name
          </th>
          <th class="p-6 text-sm font-semibold text-foreground-secondary uppercase tracking-wider">
            App Name
          </th>
          <th class="p-6 text-sm font-semibold text-foreground-secondary uppercase tracking-wider">
            API Key
          </th>
          <th class="p-6 text-sm font-semibold text-foreground-secondary uppercase tracking-wider">
            API URL
          </th>
          <th class="p-6 text-sm font-semibold text-foreground-secondary uppercase tracking-wider">
            Description
          </th>
          <th class="p-6 text-center text-sm font-semibold text-foreground-secondary uppercase tracking-wider">
            Actions
          </th>
        </tr>
      </thead>
      <tbody>
        {#each $filteredKeys as key (key.id)}
          <tr class="border-b border-border last:border-0 hover:bg-background-elevated/50 transition-colors">
            <td class="p-6 font-medium text-foreground text-sm">
              <EditableCell 
                value={key.keyName} 
                onSave={(v) => vaultStore.saveKey({ id: key.id, keyName: v })}
                label="Key Name"
              />
            </td>
            <td class="p-6 text-sm text-foreground">
              <EditableCell 
                value={key.appName || ''} 
                onSave={(v) => vaultStore.saveKey({ id: key.id, appName: v })}
                label="App Name"
                placeholder="-"
              />
            </td>
            <td class="p-6">
              <code class="text-xs text-foreground-secondary font-mono bg-background-subtle px-1.5 py-0.5 rounded">
                {'•'.repeat(Math.min(key.keyValue?.length || 8, 12))}
              </code>
            </td>
            <td class="p-6 text-sm text-foreground-secondary font-mono text-xs max-w-[200px]">
              <EditableCell 
                value={key.apiUrl || ''} 
                onSave={(v) => vaultStore.saveKey({ id: key.id, apiUrl: v })}
                label="API URL"
                placeholder="-"
              />
            </td>
            <td class="p-6 text-sm text-foreground-secondary max-w-[200px]">
                <EditableCell 
                    value={key.description || ''} 
                    onSave={(v) => vaultStore.saveKey({ id: key.id, description: v })}
                    label="Description"
                    placeholder="-"
                />
            </td>
            <td class="p-6 text-center">
              <KeyActions {key} onToggleExpand={() => toggleExpand(key.id)} />
            </td>
          </tr>
          {#if expandedKeyId === key.id}
          <tr class="bg-background-subtle/50">
             <td colspan="6" class="p-6">
                <!-- Expanded Detail View -->
                <div class="grid grid-cols-2 gap-8 text-sm">
                    <div>
                        <div class="mb-4">
                            <span class="text-xs font-semibold text-foreground-secondary uppercase block mb-1">Full API Key</span>
                            <div class="flex items-center gap-2">
                                <code class="bg-background border border-border rounded px-2 py-1 font-mono text-foreground break-all select-all">
                                    {decryptedValues[key.id] || "•••••••••••••••••••••"}
                                </code>
                                <button 
                                    class="p-1 hover:bg-background-elevated rounded text-primary transition-colors"
                                    onclick={() => toggleReveal(key.id)}
                                    aria-label={decryptedValues[key.id] ? 'Hide key' : 'Reveal key'}
                                    title={decryptedValues[key.id] ? 'Hide key' : 'Reveal key'}
                                >
                                    {#if decryptedValues[key.id]}
                                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"/></svg>
                                    {:else}
                                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/></svg>
                                    {/if}
                                </button>
                            </div>
                        </div>
                        <div class="mb-4">
                            <span class="text-xs font-semibold text-foreground-secondary uppercase block mb-1">Timestamps</span>
                            <div class="text-foreground-secondary">
                                <p>Created: {new Date(key.createdAt || '').toLocaleString()}</p>
                                <p>Updated: {new Date(key.updatedAt || '').toLocaleString()}</p>
                            </div>
                        </div>
                    </div>
                    <div>
                       <div class="mb-4">
                            <span class="text-xs font-semibold text-foreground-secondary uppercase block mb-1">Full URL</span>
                            <p class="font-mono text-foreground break-all">{key.apiUrl || '-'}</p>
                       </div>
                       <div>
                            <span class="text-xs font-semibold text-foreground-secondary uppercase block mb-1">Description</span>
                             <p class="text-foreground whitespace-pre-wrap">{key.description || '-'}</p>
                       </div>
                    </div>
                </div>
             </td>
          </tr>
          {/if}
        {/each}
      </tbody>
    </table>
  </div>

  <!-- Mobile Card View (<512px) -->
  <div class="@lg:hidden space-y-3">
    {#each $filteredKeys as key (key.id)}
      <div class="bg-background-surface border border-border rounded-lg p-4 space-y-3 shadow-sm">
        <div class="flex justify-between items-start">
          <div class="flex-1 mr-4 min-w-0">
            <div class="font-semibold text-foreground mb-1">
              <EditableCell 
                value={key.keyName} 
                onSave={(v) => vaultStore.saveKey({ id: key.id, keyName: v })}
                label="Key Name"
              />
            </div>
            <div class="text-sm text-foreground-secondary">
              <EditableCell 
                value={key.appName || ''} 
                onSave={(v) => vaultStore.saveKey({ id: key.id, appName: v })}
                label="App Name"
                placeholder="No App Name"
              />
            </div>
          </div>
          <KeyActions {key} onToggleExpand={() => toggleExpand(key.id)} />
        </div>
        <div>
          <span class="text-xs text-foreground-secondary uppercase tracking-wider block mb-1">API Key</span>
          {#if expandedKeyId === key.id}
             <div class="flex flex-col gap-1 items-start">
                 <div class="flex items-center gap-2 w-full">
                     <code class="text-sm font-mono text-foreground bg-background-subtle px-2 py-1 rounded break-all select-all flex-1">
                        {decryptedValues[key.id] || "•••••••••••••••••••••"}
                     </code>
                 </div>
                 <button 
                    class="p-1.5 hover:bg-background-elevated rounded text-primary transition-colors"
                    onclick={() => toggleReveal(key.id)}
                    aria-label={decryptedValues[key.id] ? 'Hide key' : 'Reveal key'}
                    title={decryptedValues[key.id] ? 'Hide key' : 'Reveal key'}
                 >
                    {#if decryptedValues[key.id]}
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"/></svg>
                    {:else}
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/></svg>
                    {/if}
                 </button>
             </div>
          {:else}
            <code class="text-sm font-mono text-foreground bg-background-subtle px-2 py-1 rounded inline-block">
                {'•'.repeat(Math.min(key.keyValue?.length || 8, 12))}
            </code>
          {/if}
        </div>
        
        <div>
          <span class="text-xs text-foreground-secondary uppercase tracking-wider block mb-1">API URL</span>
          <div class="text-sm text-foreground">
            <EditableCell 
              value={key.apiUrl || ''} 
              onSave={(v) => vaultStore.saveKey({ id: key.id, apiUrl: v })}
              label="API URL"
              placeholder="Add URL..."
            />
          </div>
        </div>
        
        <div>
          <span class="text-xs text-foreground-secondary uppercase tracking-wider block mb-1">Description</span>
          <div class="text-sm text-foreground">
            <EditableCell 
              value={key.description || ''} 
              onSave={(v) => vaultStore.saveKey({ id: key.id, description: v })}
              label="Description"
              placeholder="Add description..."
            />
          </div>
        </div>
        
        {#if expandedKeyId === key.id}
        <div class="pt-3 border-t border-border mt-2">
            <span class="text-xs text-foreground-secondary uppercase tracking-wider block mb-2">Timestamps</span>
            <div class="text-xs text-foreground-secondary flex flex-col gap-1">
                <div class="flex justify-between">
                    <span>Created:</span>
                    <span class="font-mono">{new Date(key.createdAt || '').toLocaleString()}</span>
                </div>
                <div class="flex justify-between">
                    <span>Updated:</span>
                    <span class="font-mono">{new Date(key.updatedAt || '').toLocaleString()}</span>
                </div>
            </div>
        </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
