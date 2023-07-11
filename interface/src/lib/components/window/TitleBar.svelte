<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
</script>

{#await invoke('get_platform') then platform}
  {#if platform === 'Windows'}
    <div class="">{platform}</div>
  {/if}
{/await}
