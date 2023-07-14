<script lang="ts">
  import About from '$lib/components/window/dialogs/About.svelte';
  import Main from '$lib/components/window/main/Main.svelte';
  import TitleBar from '../lib/components/window/titlebar/TitleBar.svelte';
</script>

<TitleBar />
<Main><slot /></Main>
<About />
