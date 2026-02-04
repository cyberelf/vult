<script lang="ts">
  import { filteredKeys } from '$lib/stores/vault';
  import { uiStore } from '$lib/stores/ui';
  import KeyActions from './KeyActions.svelte';
</script>

<div class="rounded-md border bg-card overflow-hidden">
  <table class="w-full hidden md:table">
    <thead class="bg-muted/50 border-b">
      <tr>
        <th class="p-4 text-left text-sm font-semibold text-muted-foreground uppercase">
          Key Name
        </th>
        <th class="p-4 text-left text-sm font-semibold text-muted-foreground uppercase">
          App Name
        </th>
        <th class="p-4 text-left text-sm font-semibold text-muted-foreground uppercase">
          API URL
        </th>
        <th class="p-4 text-left text-sm font-semibold text-muted-foreground uppercase">
          Description
        </th>
        <th class="p-4 text-center text-sm font-semibold text-muted-foreground uppercase w-[200px]">
          Actions
        </th>
      </tr>
    </thead>
    <tbody>
      {#each $filteredKeys as key (key.id)}
        <tr class="border-b hover:bg-muted/50">
          <td class="p-4 font-medium">{key.key_name}</td>
          <td class="p-4">{key.app_name}</td>
          <td class="p-4 text-sm text-muted-foreground">
            {key.api_url || '-'}
          </td>
          <td class="p-4 text-sm text-muted-foreground">
            {key.description || '-'}
          </td>
          <td class="p-4 text-center">
            <KeyActions {key} />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>

  <!-- Mobile Card View -->
  <div class="md:hidden space-y-4 p-4">
    {#each $filteredKeys as key (key.id)}
      <div class="bg-card border rounded-lg p-4 space-y-3">
        <div class="flex justify-between items-start">
          <div>
            <h4 class="font-semibold">{key.key_name}</h4>
            <p class="text-sm text-muted-foreground">{key.app_name}</p>
          </div>
          <KeyActions {key} />
        </div>
        {#if key.api_url}
          <div>
            <span class="text-xs text-muted-foreground">API URL:</span>
            <p class="text-sm">{key.api_url}</p>
          </div>
        {/if}
        {#if key.description}
          <div>
            <span class="text-xs text-muted-foreground">Description:</span>
            <p class="text-sm">{key.description}</p>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
