<script>
  import Bottom from './Bottom.svelte';
  import Nav from './Nav.svelte';
</script>

<main class="w-full h-[calc(100%_-_30px)] p-2 flex flex-col space-y-2">
  <div class="flex w-full h-full space-x-2">
    <Nav />
    <div class="flex-1 h-full w-full rounded-lg bg-[#121212]">
      <slot />
    </div>
  </div>
  <Bottom />
</main>
