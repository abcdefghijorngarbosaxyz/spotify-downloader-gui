<script lang="ts">
  import { useLocalStorage } from '$lib/utils/local-storage';
  import { Menu, MenuButton, MenuItem, MenuItems } from '@rgossiaux/svelte-headlessui';
  import { invoke } from '@tauri-apps/api/tauri';
  import { writable } from 'svelte/store';

  import { ABOUT_DIALOG_OPEN } from '../../../../store';
  import { onMount } from 'svelte';

  const IS_A_MENU_OPEN = writable<boolean>(false);
  const OPEN_MENU = writable<string | null>(null);

  const ALWAYS_ON_TOP = writable<boolean>(false);

  const AppMenu: Array<{
    [key: string]: Array<{
      name: string;
      action: string;
      accelerator?: string;
      seperateAfter?: boolean;
      disabled?: boolean;
      selected?: boolean;
    }>;
  }> = [
    {
      File: [
        {
          name: 'Open download folder...',
          action: 'open_download_folder',
          accelerator: 'Ctrl+O'
        },
        {
          name: 'Select download folder...',
          action: 'select_download_folder',
          accelerator: 'Ctrl+Shift+O',
          seperateAfter: true
        },
        {
          name: 'Exit',
          action: 'close_window',
          accelerator: 'Alt+F4'
        }
      ]
    },
    {
      View: [
        {
          name: 'Toggle developer tools',
          action: 'devtools',
          accelerator: 'Ctrl+Shift+I'
        }
      ]
    },
    {
      Window: [
        {
          name: 'Minimize',
          action: 'minimize',
          seperateAfter: true
        },
        {
          name: 'Always on top',
          action: 'always_on_top',
          selected: $ALWAYS_ON_TOP
        }
      ]
    },
    {
      Help: [
        {
          name: 'Documentation',
          action: 'docs'
        },
        {
          name: 'Show release notes',
          action: 'release_notes',
          seperateAfter: true
        },
        {
          name: 'Report issue',
          action: 'report_issue'
        },
        {
          name: 'Join us on Discord',
          action: 'join_us_on_discord',
          seperateAfter: true
        },
        {
          name: 'Check for updates...',
          action: 'check_for_updates',
          seperateAfter: true
        },
        {
          name: 'About',
          action: 'about_window'
        }
      ]
    }
  ];

  const handleMenuItemClicked = async (action: string) => {
    IS_A_MENU_OPEN.set(false);

    // Handle invokes that returns promises with value
    switch (action) {
      case 'about_window':
        {
          ABOUT_DIALOG_OPEN.set(true);
        }
        break;
      case 'always_on_top':
        {
          await invoke<boolean>(action)
            .then((response) => {
              ALWAYS_ON_TOP.set(response);

              for (const menu of AppMenu) {
                for (const [key, items] of Object.entries(menu)) {
                  if (key === 'Window') {
                    for (const item of items) {
                      if (item.action === 'always_on_top') {
                        item.selected = response;
                        break;
                      }
                    }
                  }
                }
              }
            })
            .catch(console.error);
        }
        break;
      default:
        await invoke(action).catch(console.error);
    }
  };

  const isAlwaysOnTop = async () => {
    await invoke<boolean>('is_always_on_top')
      .then((value) => ALWAYS_ON_TOP.set(value))
      .catch(console.error);
  };

  onMount(isAlwaysOnTop);
</script>

{#each AppMenu as menu}
  {#each Object.entries(menu) as [key, items]}
    <Menu class="relative text-xs h-[30px]">
      <MenuButton
        on:click={() => IS_A_MENU_OPEN.set(!$IS_A_MENU_OPEN)}
        on:mouseover={() => OPEN_MENU.set(key)}
        as="button"
        class={`px-2 h-full flex items-center hover:text-white hover:bg-[rgb(41,_42,_45)] focus:bg-[rgb(41,_42,_45)] cursor-default ${
          $OPEN_MENU === key && $IS_A_MENU_OPEN && 'bg-[rgb(41,_42,_45)]'
        }`}>
        {key}
      </MenuButton>
      {#if $OPEN_MENU === key && $IS_A_MENU_OPEN}
        <MenuItems
          static
          class="absolute top-[30px] z-[20] left-0 bg-[rgb(41,_42,_45)] border border-[rgb(63,_64,_66)] py-[3px]">
          {#each items as item}
            <MenuItem
              class="hover:bg-[rgb(63,_64,_66)] hover:text-white flex items-center pr-[24px] pl-[8px] py-[4px]"
              on:click={() => handleMenuItemClicked(item.action)}>
              <div
                class="h-[16px] w-[16px] mr-[12px] flex items-center justify-center text-white/75">
                {#if item.selected}
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    width="24"
                    height="24"
                    fill="currentColor">
                    <path
                      d="M21.03 5.72a.75.75 0 0 1 0 1.06l-11.5 11.5a.747.747 0 0 1-1.072-.012l-5.5-5.75a.75.75 0 1 1 1.084-1.036l4.97 5.195L19.97 5.72a.75.75 0 0 1 1.06 0Z" />
                  </svg>
                {/if}
              </div>
              <div class="flex-1 whitespace-nowrap">{item.name}</div>
              {#if item.accelerator}
                <div class="flex-none ml-[12px] flex whitespace-nowrap text-end text-white/50">
                  {item.accelerator}
                </div>
              {/if}
            </MenuItem>
            {#if item.seperateAfter}
              <div class="bg-[rgb(63,_64,_66)] h-[1px] my-[5px] w-full" />
            {/if}
          {/each}
        </MenuItems>
      {/if}
    </Menu>
  {/each}
{/each}

<!-- Closes the open menu when clicked outside -->
{#if $IS_A_MENU_OPEN}
  <button
    on:click={() => IS_A_MENU_OPEN.set(false)}
    class="fixed h-[calc(100vh_-_30px)] z-[10] w-screen top-[30px] left-0 cursor-default" />
{/if}
