<script lang="ts">
    import { MediaTrendSort, PageIndex, type AnimeCardInfo, BodyScroll, ClientWSMessageType, ServerWSMessageType, Websocket, CachedValues, Page, SearchPageStore } from "$lib";
    import { onDestroy, onMount } from "svelte";
    import FillerSpace from "../../layout/FillerSpace.svelte";
    import AnimeWrapper from "../../layout/AnimeWrapper.svelte";
    import { get } from "svelte/store";

    export let page: PageIndex;

    let animes: AnimeCardInfo[] = [];
    let hasNextPage: boolean = true;
    let totalPages: number = 0;

    let animeWrapperHeight = 0;
    let pageHeight = 0;
    let ready = false;
    let readyForNextPage = true;
    
    let type: MediaTrendSort | "season" | "search" | null = null;

    onDestroy(() => {
        animes = [];
        hasNextPage = true;
        totalPages = 0;
        ready = false;
        readyForNextPage = true;
        type = null;
    })

    onMount(async () => {
        let cached: {data: typeof animes, lastUpdated: Date} | null = null;
        totalPages = 1;

        switch(page) {
            case PageIndex.TRENDING:
                cached = await CachedValues.getPage(Page.Trending);
                type = MediaTrendSort.TRENDING_DESC;
                break;
            case PageIndex.POPULAR:
                cached = await CachedValues.getPage(Page.Popular);
                type = MediaTrendSort.POPULARITY_DESC;
                break;
            case PageIndex.TOP:
                cached = await CachedValues.getPage(Page.Top);
                type = MediaTrendSort.SCORE_DESC;
                break;
            case PageIndex.SEASON:
                cached = await CachedValues.getPage(Page.Seasonal);
                type = "season";
                break;
            case PageIndex.SEARCH:
                type = "search";
                break;
            default: {
                console.error("Invalid page index");
                return;
            }
        }
        if (type == null) return;
        if (cached != null) {
            const date = new Date();
            if (date.getTime() - cached.lastUpdated.getTime() > 1000 * 60 * 60) {
                return await refreshAnimeList(1);
            }
            animes = cached.data;
            totalPages = 2;
            return ready = true;
        }

        await refreshAnimeList(1);
        ready = true;
    });

    if (page == PageIndex.SEARCH) {
        Websocket.addListener<{list: AnimeCardInfo[], hasNextPage: boolean}>(ServerWSMessageType.SearchPageContent, message => {
            animes = message.list;
            hasNextPage = message.hasNextPage;
            readyForNextPage = true;
            CachedValues.setPage(Page.Search, animes);
        })
    } else {
        Websocket.addListener<{media: AnimeCardInfo[], hasNextPage: boolean}>(ServerWSMessageType.CommonPageData, message => {
            animes = [...animes, ...message.media];
            hasNextPage = message.hasNextPage;
            if (totalPages == 1) {
                switch(page) {
                    case PageIndex.TRENDING:
                        CachedValues.setPage(Page.Trending, animes);
                        break;
                    case PageIndex.POPULAR:
                        CachedValues.setPage(Page.Popular, animes);
                        break;
                    case PageIndex.TOP:
                        CachedValues.setPage(Page.Top, animes);
                        break;
                    case PageIndex.SEASON:
                        CachedValues.setPage(Page.Seasonal, animes);
                    break;
                }
                totalPages = 2;
            } else {
                totalPages++;
            }
    
            readyForNextPage = true;
        })
    }

    const refreshAnimeList = async (pageNumber: number) => {
        let seasonCheck = type === "season";
        let searchCheck = type === "search";
        await Websocket.send(ClientWSMessageType.CommonPage, {
            page: pageNumber,
            date: seasonCheck ? new Date().toISOString() : undefined,
            genres: [],
            blacklistedGenres: [],
            sort: seasonCheck ? [MediaTrendSort.TRENDING_DESC, MediaTrendSort.POPULARITY_DESC] : [type],
            search: searchCheck ? get(SearchPageStore)?.value! : undefined,
        });
    }

    BodyScroll.subscribe(async (value) => {
        if (!ready) return;
        if (!hasNextPage) return;
        if (totalPages == 0) return;
        const currentPageCount = totalPages - 1
        const middleOfCurrentPage = (pageHeight / currentPageCount) * 2 / 3;
        if (readyForNextPage && value > pageHeight - (middleOfCurrentPage)) {
            readyForNextPage = false;
            await refreshAnimeList(totalPages);
        }
    })
    

    
</script>

<main>
    <section class="options">

    </section>
    <FillerSpace height="10vh" />
    <section class="anime_list" bind:clientHeight={pageHeight}>
        {#if animes}
            {#each animes as anime}
                <div>
                    <AnimeWrapper anime={anime} bind:height={animeWrapperHeight}/>
                </div>
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

    main .options {
        display: flex;
        justify-content: center;
        height: 10vh;
        position: fixed;
    }

    main .anime_list {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 1rem;
        padding: 1rem;
        overflow-y: scroll;
        position: relative;
    }

    main .anime_list div:last-child {
        margin-bottom: 10vh;
    }
</style>