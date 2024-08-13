<script lang="ts">
    import { MediaTrendSort, PageIndex, type AnimeCardInfo, TrendingPageStore, findScroller, BodyScroll } from "$lib";
    import { onMount } from "svelte";
    import FillerSpace from "../../layout/FillerSpace.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import AnimeWrapper from "../../layout/AnimeWrapper.svelte";
    import { get } from "svelte/store";
    import type { UIEventHandler } from "svelte/elements";    
  import { PopularPageStore, TopPageStore } from "$lib/store";

    export let page: PageIndex;

    let animes: AnimeCardInfo[] = [];
    let hasNextPage: boolean = true;
    let totalPages: number = 1;

    let scrollY = 0;
    let section: HTMLElement;
    let animeWrapperHeight = 0;
    let pageHeight = 0;
    let ready = false;
    

    onMount(async () => {
        let stored: AnimeCardInfo[] | null = null;
        switch(page) {
            case PageIndex.TRENDING:
                stored = get(TrendingPageStore);
                break;
            case PageIndex.POPULAR:
                stored = get(PopularPageStore)
                break;
            case PageIndex.TOP:
                stored = get(TopPageStore);
                break;
            case PageIndex.SEASON:
                break;
            default: {
                return;
            }
        }

        if (stored != null) {
            animes = stored;
            totalPages = 1;
            return;
        }
        await refreshAnimeList(1);
        if (stored == null) {
            switch(page) {
                case PageIndex.TRENDING:
                    TrendingPageStore.set(animes);
                    break;
                case PageIndex.POPULAR:
                    PopularPageStore.set(animes);
                    break;
                case PageIndex.TOP:
                    TopPageStore.set(animes);
                    break;
            }
            totalPages = 1;
        }
        ready = true;
    });

    const refreshAnimeList = async (pageNumber: number) => {
        switch(page) {
            case PageIndex.TRENDING:
                animes = [...animes, ...(await getData(MediaTrendSort.TRENDING_DESC, pageNumber)).data];
                break;
            case PageIndex.POPULAR:
                animes = [...animes, ...(await getData(MediaTrendSort.POPULARITY_DESC, pageNumber)).data];
                break;
            case PageIndex.TOP:
                animes = [...animes, ...(await getData(MediaTrendSort.SCORE_DESC, pageNumber)).data];
                break;
            case PageIndex.SEASON:
                // animes = [...animes, ...(await getData(MediaTrendSort., pageNumber)).data];
                break;
        }
        
    }

    const getData = async (path: MediaTrendSort, page: number) => {
        
        if (!hasNextPage) return { data: [], hasNextPage: false };
        const path_lowercase = path.toLowerCase();
        const data = await invoke("get_page", { path: path_lowercase, page, genres: [], blacklistedGenres: [] }) as { data: AnimeCardInfo[], hasNextPage: boolean };
        
        // const data = await (async () => {
        //     const url = new URL(`http://127.0.0.1:8787/anime/pages/${path_lowercase}`);
        //     url.searchParams.append('page', page.toString());
        //     url.searchParams.append('genres', "");
        //     url.searchParams.append('blacklisted', "");
        //     return await (await fetch(url, { method: 'GET',  mode: 'cors', })).json()
        // })()

        hasNextPage = data.hasNextPage;
        return data as { data: AnimeCardInfo[], hasNextPage: boolean }
    }

    BodyScroll.subscribe(async (value) => {
        if (!ready) return;
        if (!hasNextPage) return;
        const middleOfCurrentPage = (pageHeight / totalPages) / 2
        if (value > pageHeight - middleOfCurrentPage) {
            totalPages++;
            await refreshAnimeList(totalPages);
        }
    })
    

    
</script>

<!-- <svelte:window bind:this={section}/> -->
<main>
    <section bind:clientHeight={pageHeight}>
        {#if animes}
            {#each animes as anime}
                <AnimeWrapper anime={anime} bind:height={animeWrapperHeight}/>
            {/each}
        {/if}
    </section>
</main>
<FillerSpace height="10vh" />
<style>
    main {
        width: 100vw;
        background: var(--background);
    }

    main section {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 1rem;
        padding: 1rem;
        overflow-y: scroll;
    }
</style>