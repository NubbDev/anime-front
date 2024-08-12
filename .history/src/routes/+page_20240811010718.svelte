<script lang="ts">
  import { onMount } from "svelte";
  import NavBar from "../components/navigation/NavBar.svelte";
  import { PageIndex, PageStore, type AnimeCardInfo } from "$lib";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Trending from "../components/pages/main/Trending.svelte";

  const frontpage: {
    trending: AnimeCardInfo[],
    popular: AnimeCardInfo[],
    season: AnimeCardInfo[],
    top: AnimeCardInfo[],
  } = {}

  onMount(async () => {
    const data = await invoke("get_frontpage");
  });

</script>
<NavBar />
<SearchNoti/>
<Trending />