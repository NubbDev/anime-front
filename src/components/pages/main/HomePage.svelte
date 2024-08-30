<script lang="ts">
    import { type AnimeCardInfo, AppStateStore, AppStates, ClientWSMessageType, ServerWSMessageType, Websocket, CachedValues, Page } from "$lib";
    import { get } from "svelte/store";

    import { onDestroy, onMount } from "svelte";

    import AnimeSections from './AnimeSections.svelte'
    import BannerCarousel from './BannerCarousel.svelte'
    import FillerSpace from "../../layout/FillerSpace.svelte";

    let frontpage: {
        trending: AnimeCardInfo[],
        popular: AnimeCardInfo[],
        season: AnimeCardInfo[],
        top: AnimeCardInfo[],
    } | null = null;

    onMount(async() => {
        await getData();
    })

    onDestroy(() => {
        frontpage = null;
    })

    const getData = async () => {
        const cached = await CachedValues.getPage<typeof frontpage>(Page.Home);
        if (cached != null) {
            console.log('cached')
            const date = new Date();
            if (date.getTime() - cached.lastUpdated.getTime() > 1000 * 60 * 10) {
                await Websocket.send(ClientWSMessageType.HomePage);
                return
            }
            frontpage = cached.data;
            if (get(AppStateStore) == AppStates.CONNECTED) {
                AppStateStore.set(AppStates.READY);
            }
            return 
        }
        await Websocket.send(ClientWSMessageType.HomePage);
    }

    Websocket.addListener<{ trending: AnimeCardInfo[], popular: AnimeCardInfo[], season: AnimeCardInfo[], top: AnimeCardInfo[]}>(ServerWSMessageType.HomePageRoute, async (message) => {
        frontpage = message;
        await CachedValues.setPage(Page.Home, frontpage);
        if (get(AppStateStore) == AppStates.CONNECTED) {
            AppStateStore.set(AppStates.READY);
        }
    })

    AppStateStore.subscribe(async value => {
        if (value == AppStates.CONNECTED) {
            return await getData();
        }
    })
</script>

{#if frontpage}
    <BannerCarousel anime={frontpage.season} />
    <div>
        <AnimeSections title="Trending" animes={frontpage.trending} />
        <AnimeSections title="Popular" animes={frontpage.popular} />
        <AnimeSections title="Top" animes={frontpage.top} />
        <FillerSpace height="7vh" />
    </div>
{/if}

<style>
    div {
        position: relative;
        background: var(--background);
        z-index: 1;
    }
</style>