<script lang="ts">
    import { onMount } from "svelte";
    import { PageIndex, PageStore, type AnimeCardInfo } from "$lib";
    import { invoke } from "@tauri-apps/api/core";
    
    let TrendingAnimes: AnimeCardInfo[] = [];
    onMount(async () => {
        const data: AnimeCardInfo[] = await invoke("get_trending", { page: "" });
        TrendingAnimes.push(...data);
    });
</script>

<h1>Trending</h1>
{#each TrendingAnimes as anime}
    <div>
        <img src={anime.coverImage.large} alt={anime.title.english} />
        
    </div>
{/each}