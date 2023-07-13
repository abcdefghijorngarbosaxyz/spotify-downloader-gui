<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { ABOUT_DIALOG_OPEN } from '../../../../store';
  import { onMount } from 'svelte';
  import AppIcon from '../../../../../../assets/icons/128x128.png';

  const openLicense = async () => {
    await invoke('open_license');
  };

  const getAppInfo = async () =>
    await invoke<{ version: string; commit: string; platform: string; bit: string }>('app_info');
</script>

{#if $ABOUT_DIALOG_OPEN}
  {#await getAppInfo() then appInfo}
    <div class="fixed left-0 text-xs w-screen h-screen top-0 bg-[#040404]/50 flex items-center">
      <button
        tabindex="-1"
        class="w-full h-full absolute top-0 left-0 cursor-default"
        on:click={() => ABOUT_DIALOG_OPEN.set(false)} />
      <div
        class="w-[400px] select-none h-[400px] flex flex-col justify-center items-center relative mx-auto my-auto shadow-md bg-[#181818]">
        <button
          tabindex="-1"
          on:click={() => ($ABOUT_DIALOG_OPEN = false)}
          class="w-[46px] h-[30px] inline-flex justify-center transition-colors duration-200 items-center absolute z-[20] select-none bg-[#040404] hover:bg-[rgba(232,_17,_35,_0.9)] active:bg-[rgba(232,_17,_35,_0.6)] right-0 top-0">
          <svg
            stroke="currentColor"
            fill="currentColor"
            stroke-width="0"
            viewBox="0 0 16 16"
            class="w-[16px] h-[16px]"
            xmlns="http://www.w3.org/2000/svg">
            <path
              d="M7.116 8l-4.558 4.558.884.884L8 8.884l4.558 4.558.884-.884L8.884 8l4.558-4.558-.884-.884L8 7.116 3.442 2.558l-.884.884L7.116 8z" />
          </svg>
        </button>
        <img src={AppIcon} width="128" height="128" alt="app_icon" />
        <h1 class="font-semibold my-4 text-sm">
          spotDL GUI for {appInfo.platform}
          {`(${appInfo.bit})`}
        </h1>
        <p>Version {appInfo.version}-{appInfo.commit}</p>

        <!-- To implement open on rust -->
        <button class="hover:underline focus:underline decoration-blue-500 mt-1 text-blue-500">
          Changelog
        </button>

        <!-- To implement open on rust -->
        <button class="hover:underline focus:underline decoration-blue-500 mt-1 text-blue-500">
          License and Open Source Notices
        </button>
      </div>
    </div>
  {/await}
{/if}
