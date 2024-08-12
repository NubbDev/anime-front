<script lang="ts">
  import { onMount } from "svelte";
  import NavBar from "../components/navigation/NavBar.svelte";
  import { PageIndex, PageStore, type AnimeCardInfo } from "$lib";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import AnimeSections from "../components/pages/main/AnimeSections.svelte";
  import BannerAnime from "../components/pages/main/BannerAnime.svelte";

  let frontpage: {
    trending: AnimeCardInfo[],
    popular: AnimeCardInfo[],
    season: AnimeCardInfo[],
    top: AnimeCardInfo[],
  }
  let bannerAnime: AnimeCardInfo;

  onMount(async () => {
    // const data = await invoke("get_frontpage") as typeof frontpage; 
    const response = await fetch('http://127.0.0.1:8787/anime/frontpage', {
      method: 'GET',
      mode: 'cors',
      // headers: {
      //   "Access-Control-Allow-Origin": "*",
      // }
      // headers: {
      //   'Content-Type': 'application/json'
      // }
    });
    const data = await response.json();
    for (const anime in data.trending) {
      if (data.trending[anime].bannerImage) {
        bannerAnime = data.trending[anime];
        break;
      }
    } 
    frontpage = data;
  });

</script>
<NavBar />
<SearchNoti/>
{#if bannerAnime}
  <BannerAnime anime={bannerAnime} />
{/if}

{#if frontpage}
  <!-- <AnimeSections title="Trending" animes={frontpage.trending} />
  <AnimeSections title="Popular" animes={frontpage.popular} />
  <AnimeSections title="Season" animes={frontpage.season} />
  <AnimeSections title="Top" animes={frontpage.top} /> -->
{/if}