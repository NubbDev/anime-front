<script lang="ts">
    import { onMount } from "svelte";
    import { PageIndex, PageStore, type AnimeCardInfo } from "$lib";
    import { invoke } from "@tauri-apps/api/core";
  import AnimeSections from "./AnimeSections.svelte";
    
    let trendingAnimes: AnimeCardInfo[] = [];
    onMount(async () => {
        const data: AnimeCardInfo[] = await invoke("get_trending", { page: "" });
        trendingAnimes = data;
    });
</script>

<AnimeSections command_name="get_trending" title="Trending" />
