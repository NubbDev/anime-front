<script lang="ts">
    import { type AnimeCardInfo, FrontPageStore } from "$lib";
    import { get } from "svelte/store";

    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    import AnimeSections from './AnimeSections.svelte'
    import BannerCarousel from './BannerCarousel.svelte'
  import FillerSpace from "../../layout/FillerSpace.svelte";

    let frontpage: {
        trending: AnimeCardInfo[],
        popular: AnimeCardInfo[],
        season: AnimeCardInfo[],
        top: AnimeCardInfo[],
    }

    const getData = async () => {
        const data = await invoke("get_frontpage") as typeof frontpage;

        // const data = await (await fetch('http://127.0.0.1:8787/anime/frontpage', { method: 'GET',  mode: 'cors', })).json()
        return data as typeof frontpage;
    }

    onMount(async () => {
        const store = get(FrontPageStore);
        if (store !== null) {
            const date = new Date();
            const lastUpdate = new Date(store.lastUpdated);
            // if the last update was more than 1 hour ago, update the frontpage
            if (date.getTime() - lastUpdate.getTime() > 3600000) {
                return frontpage = await getData();
            }
            return frontpage = store;
        }

        frontpage = await getData();
        FrontPageStore.set({ ...frontpage, lastUpdated: new Date() });
    });
</script>

{#if frontpage}
    <BannerCarousel anime={frontpage.season} />
    <AnimeSections title="Trending" animes={frontpage.trending} />
    <AnimeSections title="Popular" animes={frontpage.popular} />
    <AnimeSections title="Top" animes={frontpage.top} />
{/if}
<FillerSpace height="10vh" />