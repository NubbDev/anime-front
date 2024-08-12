<script lang="ts">
    import { onMount } from "svelte";
    import { PageIndex, PageStore, type AnimeCardInfo } from "$lib";
    import { invoke } from "@tauri-apps/api/core";
    
    const TrendingAnimes: AnimeCardInfo[] = [];
    let loaded = false;
    onMount(async () => {
        const data: AnimeCardInfo[] = await invoke("get_trending", { page: "" });
        TrendingAnimes.push(...data);
        loaded = true;
    });
</script>

<h1>Trending</h1>
{#if !loaded}
    <p>Loading...</p>
{:else}
    {#each TrendingAnimes as anime}
        <div>
            <img src={anime.coverImage.large} alt={anime.title.english} />
        </div>
    {/each}
{/if}
/