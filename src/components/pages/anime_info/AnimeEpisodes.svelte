<script lang="ts">
    import type { GoGoAnimeInfo } from "$lib";
  import FillerSpace from "../../layout/FillerSpace.svelte";
  import AnimeEpisode from "./AnimeEpisode.svelte";

    export let anime: {
        sub: GoGoAnimeInfo | null;
        dub: GoGoAnimeInfo | null;
    } | undefined;

    export let episodesReady: boolean = false;

</script>

<section>
{#if episodesReady && anime && anime.sub}
    {#each anime.sub.episodes as episode, i}
        <AnimeEpisode delay={i * 0.1} sub={episode} dub={anime.dub ? anime.dub.episodes[i] ?? undefined : undefined} />
    {/each}
    <FillerSpace height="7vh"/>
{:else}
    <div class="loader"/>
{/if}
</section>

<style>
    section {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 1rem;
        padding: 1rem;
        box-sizing: border-box;
    }
    .loader {
        width: 50px;
        aspect-ratio: 1;
        border-radius: 50%;
        background: 
            radial-gradient(farthest-side,#ffa516 94%,#0000) top/8px 8px no-repeat,
            conic-gradient(#0000 30%,#ffa516);
        mask: radial-gradient(farthest-side,#0000 calc(100% - 8px),#000 0);
        animation: l13 1s infinite linear;
    }
    @keyframes l13{ 
        100%{transform: rotate(1turn)}
    }
</style>