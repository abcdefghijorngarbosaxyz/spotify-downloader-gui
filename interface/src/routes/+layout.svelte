<script lang="ts">
  import About from '$lib/components/window/dialogs/About.svelte';
  import TitleBar from '../lib/components/window/titlebar/TitleBar.svelte';
</script>

<TitleBar />
<slot />
<About />
