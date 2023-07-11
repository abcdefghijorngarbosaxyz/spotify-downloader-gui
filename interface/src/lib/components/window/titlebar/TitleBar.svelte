<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { appWindow } from '@tauri-apps/api/window';

  const onMinimizeButtonClicked = async () => {
    await appWindow.minimize();
  };

  const onCloseButtonClicked = async () => {
    await invoke('close_window');
  };
</script>

{#await invoke('get_platform') then platform}
  {#if platform === 'Windows'}
    <div id="titlebar">
      <div data-tauri-drag-region />
      <div>
        <button on:click={onMinimizeButtonClicked}>
          <svg
            stroke="currentColor"
            fill="currentColor"
            stroke-width="0"
            viewBox="0 0 16 16"
            xmlns="http://www.w3.org/2000/svg"
            ><path d="M14 8v1H3V8h11z" />
          </svg>
        </button>
        <button>
          <svg
            stroke="currentColor"
            fill="currentColor"
            stroke-width="0"
            viewBox="0 0 16 16"
            xmlns="http://www.w3.org/2000/svg"
            ><path d="M3 3v10h10V3H3zm9 9H4V4h8v8z" />
          </svg>
        </button>
        <button on:click={onCloseButtonClicked}>
          <svg
            stroke="currentColor"
            fill="currentColor"
            stroke-width="0"
            viewBox="0 0 16 16"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M7.116 8l-4.558 4.558.884.884L8 8.884l4.558 4.558.884-.884L8.884 8l4.558-4.558-.884-.884L8 7.116 3.442 2.558l-.884.884L7.116 8z"
            />
          </svg>
        </button>
      </div>
    </div>
  {/if}
{/await}

<style lang="postcss" scoped>
  #titlebar {
    @apply flex h-[30px] w-full select-none justify-between;

    & > div:nth-of-type(2) {
      @apply flex justify-end bg-[#040404];

      & > button {
        @apply inline-flex w-[46px] items-center justify-center transition-colors duration-200;

        & > svg {
          @apply h-[16px] w-[16px];
        }
      }

      & > button:not(:nth-of-type(2)) {
        @apply text-white;
      }

      /**
      * Can use tailwind hover: active: here.
      * It works, but Svelte preprocess gives lint error
      * on my editor, so.
      */
      & > button:nth-of-type(1):hover {
        @apply bg-[rgba(255,_255,_255,_0.1)];
      }

      & > button:nth-of-type(1):active {
        @apply bg-[rgba(255,_255,_255,_0.2)];
      }

      & > button:nth-of-type(2) {
        @apply text-white/10;
      }

      & > button:nth-of-type(3):hover {
        @apply bg-[rgba(232,_17,_35,_0.9)];
      }

      & > button:nth-of-type(3):active {
        @apply bg-[rgba(232,_17,_35,_0.6)];
      }
    }
  }

  button {
    @apply cursor-default;
  }
</style>
