<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { appWindow } from '@tauri-apps/api/window';
  import AppIcon from '../../../../../../assets/generics/app_icon.svg';

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
      <div data-tauri-drag-region>
        <button tabindex="-1">
          <svg version="1.0" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
            <g
              transform="translate(0.000000,300.000000) scale(0.100000,-0.100000)"
              fill="currentColor"
              stroke="none"
            >
              <path
                d="M2148 2929 c-111 -16 -214 -84 -303 -199 -25 -32 -83 -130 -129 -218
            -103 -198 -143 -258 -230 -349 -129 -137 -270 -178 -357 -106 -119 101 -78
            336 94 528 l48 54 -53 -16 c-77 -22 -144 -50 -197 -80 -573 -330 -988 -1342
            -816 -1992 34 -126 105 -260 171 -320 225 -205 706 -235 1184 -74 283 96 508
            229 686 407 124 125 194 229 265 396 42 100 89 171 161 247 l60 62 -64 -15
            c-113 -27 -196 -61 -291 -121 -21 -13 -42 -21 -47 -18 -6 4 -10 36 -10 73 l0
            67 -28 -41 c-91 -137 -194 -201 -277 -174 -47 16 -104 66 -121 107 -32 76 -2
            165 83 244 98 92 186 128 382 159 337 51 433 143 448 430 l6 105 -65 -60
            c-249 -231 -621 -90 -713 270 -19 77 -19 234 0 310 28 107 79 197 150 263 47
            45 60 62 49 66 -9 3 -19 5 -22 5 -4 -1 -33 -5 -64 -10z m-940 -1728 c171 -36
            349 -109 372 -150 19 -36 12 -78 -16 -100 -35 -28 -54 -26 -123 8 -218 110
            -567 146 -828 85 -88 -21 -118 -14 -140 30 -15 32 -15 36 4 68 16 28 28 35 88
            51 118 30 193 36 375 32 124 -3 202 -10 268 -24z m-84 -266 c103 -21 230 -68
            297 -108 46 -27 54 -37 57 -65 4 -40 -20 -72 -55 -72 -13 0 -52 14 -86 31
            -212 108 -413 130 -704 78 -67 -12 -74 -11 -93 6 -25 23 -26 69 -2 93 16 16
            86 35 212 56 54 10 299 -3 374 -19z m4 -260 c87 -22 231 -92 249 -120 10 -16
            10 -25 0 -45 -18 -35 -46 -36 -115 -1 -175 88 -343 107 -585 64 -83 -14 -89
            -14 -107 2 -27 24 -25 61 3 79 70 46 406 59 555 21z"
              />
            </g>
          </svg>
        </button>
      </div>
      <div>
        <button tabindex="-1" on:click={onMinimizeButtonClicked}>
          <svg
            stroke="currentColor"
            fill="currentColor"
            stroke-width="0"
            viewBox="0 0 16 16"
            xmlns="http://www.w3.org/2000/svg"
            ><path d="M14 8v1H3V8h11z" />
          </svg>
        </button>
        <button tabindex="-1">
          <svg
            stroke="currentColor"
            fill="currentColor"
            stroke-width="0"
            viewBox="0 0 16 16"
            xmlns="http://www.w3.org/2000/svg"
            ><path d="M3 3v10h10V3H3zm9 9H4V4h8v8z" />
          </svg>
        </button>
        <button tabindex="-1" on:click={onCloseButtonClicked}>
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

    & > div:nth-of-type(1) {
      @apply w-full;
    }

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

  [data-tauri-drag-region] > button:has(svg) {
    @apply pointer-events-none ml-[8px] align-bottom;

    & > svg {
      @apply h-[18px] w-[18px] text-white/50;
    }
  }
</style>
