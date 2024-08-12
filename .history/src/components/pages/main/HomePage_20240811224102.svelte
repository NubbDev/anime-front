<script lang="ts">
  import type { AnimeCardInfo } from "$lib";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    let frontpage: {
        trending: AnimeCardInfo[],
        popular: AnimeCardInfo[],
        season: AnimeCardInfo[],
        top: AnimeCardInfo[],
    }
    onMount(async () => {
    const data = await invoke("get_frontpage") as typeof frontpage; 
    // const response = await fetch('http://127.0.0.1:8787/anime/frontpage', { method: 'GET',  mode: 'cors', });
    // const data = await response.json();
    for (const anime in data.trending) {
    } 
    frontpage = data;
  });
</script>

<BannerCarousel anime={frontpage.season} />
  <AnimeSections title="Trending" animes={frontpage.trending} />
  <AnimeSections title="Popular" animes={frontpage.popular} />
  <AnimeSections title="Top" animes={frontpage.top} />