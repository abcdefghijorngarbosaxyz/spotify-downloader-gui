<script lang="ts">
  import { Menu, MenuButton, MenuItem, MenuItems } from '@rgossiaux/svelte-headlessui';
  import { invoke } from '@tauri-apps/api/tauri';
  import { writable } from 'svelte/store';

  const IS_A_MENU_OPEN = writable<boolean>(false);
  const OPEN_MENU = writable<string | null>(null);

  const AppMenu: Array<{
    [key: string]: {
      name: string;
      action: string;
      accelerator?: string;
      seperateAfter?: boolean;
    }[];
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
      ],
      View: [
        {
          name: 'Toggle developer tools',
          action: 'devtools',
          accelerator: 'Ctrl+Shift+I'
        }
      ],
      Window: [
        {
          name: 'Minimize',
          action: 'minimize_window',
          seperateAfter: true
        },
        {
          name: 'Always on top',
          action: 'always_on_top'
        }
      ],
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
    await invoke(action).catch(console.error);
  };
</script>

{#each AppMenu as menu}
  {#each Object.entries(menu) as [key, items]}
    <Menu class="relative text-xs h-[30px]">
      <MenuButton
        on:click={() => IS_A_MENU_OPEN.set(!$IS_A_MENU_OPEN)}
        on:mouseover={() => OPEN_MENU.set(key)}
        as="button"
        class={`px-2 h-full flex items-center hover:bg-[rgb(41,_42,_45)] cursor-default ${
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
              class="hover:bg-[rgb(63,_64,_66)] flex items-center pr-[24px] pl-[18px] py-[5px]"
              on:click={() => handleMenuItemClicked(item.action)}>
              <div class="h-[16px] w-[16px]" />
              <div class="flex-1 whitespace-nowrap">{item.name}</div>
              {#if item.accelerator}
                <div class="flex-none ml-[12px] flex whitespace-nowrap text-end">
                  {item.accelerator}
                </div>
              {/if}
            </MenuItem>
            {#if item.seperateAfter}
              <div class="bg-[rgb(63,_64,_66)] h-[1px] my-[4px] w-full" />
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
