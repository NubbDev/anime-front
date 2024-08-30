<script lang="ts">
    import { AnimePlayerStore } from "$lib";
    import { onDestroy } from "svelte";
  import Player from "./Player.svelte";
  import FillerSpace from "../../layout/FillerSpace.svelte";



    onDestroy(() => {
        AnimePlayerStore.set(null);
    })

    AnimePlayerStore.subscribe(value => {
        if (value == null) return;
        const links = value.sources;

    })
</script>
<section>
    <FillerSpace height="7vh"/>
    <Player />
</section>

<style>
    section {
        width: 100vw;
        height: 100vh;
        background: var(--background);
        position: relative;
    }
</style>