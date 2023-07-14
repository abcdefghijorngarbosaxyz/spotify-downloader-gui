<script lang="ts">
  import { writable } from 'svelte/store';

  export const QUERY = writable<string>('');
</script>

<div class="h-[72px] w-full flex">
  <form class="w-2/3 h-full mx-auto flex py-2 relative group">
    <input
      bind:value={$QUERY}
      placeholder="What do you want to download?"
      class="w-full flex-1 peer text-white text-sm px-[48px] bg-[rgb(36,_36,_36)] focus:bg-[rgb(42,_42,_42)] hover:bg-[rgb(42,_42,_42)] rounded-full focus:outline focus:outline-white focus:outline-offset-0 hover:ring-[1px] hover:ring-offset-0 hover:ring-[rgb(65,_65,_65)]" />
    <div
      class="w-[48px] peer peer-focus:text-white h-full flex items-center justify-center absolute top-0 left-0">
      <button type="submit" class="w-[20px] h-[20px]">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="currentColor"
          viewBox="0 0 24 24"
          width="20"
          height="20">
          <path
            d="M10.25 2a8.25 8.25 0 0 1 6.34 13.53l5.69 5.69a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215l-5.69-5.69A8.25 8.25 0 1 1 10.25 2ZM3.5 10.25a6.75 6.75 0 1 0 13.5 0 6.75 6.75 0 0 0-13.5 0Z" />
        </svg>
      </button>
    </div>
    {#if $QUERY}
      <div
        class="w-[48px] peer text-white h-full flex items-center justify-center absolute top-0 right-0">
        <button type="reset" class="w-[24px] h-[24px]">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="currentColor"
            viewBox="0 0 24 24"
            width="24"
            height="24">
            <path
              d="M5.72 5.72a.75.75 0 0 1 1.06 0L12 10.94l5.22-5.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L13.06 12l5.22 5.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L12 13.06l-5.22 5.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L10.94 12 5.72 6.78a.75.75 0 0 1 0-1.06Z" />
          </svg>
        </button>
      </div>
    {/if}
  </form>
</div>
