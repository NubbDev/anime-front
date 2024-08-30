<script lang="ts">
    import { CachedValues, ServerWSMessageType, Websocket, type AnimeInfo, type GoGoAnimeInfo, AnimeSelectedStore, History } from "$lib";
    import { onDestroy, onMount } from "svelte";
    import AnimeDescription from "./AnimeDescription.svelte";
    import FillerSpace from "../../layout/FillerSpace.svelte";
    import AnimeGenres from "./AnimeGenres.svelte";

    import PlayButton from "$lib/assets/icons/playfilled.svg?component";
    import WatchTogether from "$lib/assets/icons/users-alt.svg?component";
    import Bookmark from "$lib/assets/icons/bookmark.svg?component";
  import AnimeEpisodes from "./AnimeEpisodes.svelte";

    let scrollY: number = 0;
    let wrapper: HTMLElement;
    let episodesReady: boolean = false;

    let loading = true;
    let anime: AnimeInfo | null = null;
    let title: string | null = null;
    let isDragStart = false, isDragging = false, positionDiff: number = 0, isScrolling = false, isTryingToHide = false, previousScroll = 0;
    let margin: number = 0;
    let screenY = 0;
    export let shouldHide: boolean = false;

    let wrapperWidth: number;

    onMount(() => {
        
    })

    onDestroy(() => {
        loading = true;
        anime = null;
        title = null;
        shouldHide = false;
        margin = 0;
        episodesReady = false;
    })

    AnimeSelectedStore.subscribe(value => {
        if (value) {
            anime = value;
            title = anime.title.english ?? anime.title.romaji;
            loading = false;
            episodesReady = true;
        }
    })

    Websocket.addListener<{
        sub: GoGoAnimeInfo | null,
        dub: GoGoAnimeInfo | null
    }>(ServerWSMessageType.GogoAnimeData, async (message) => {
        anime = {
            ...anime!,
            gogoanime: {
                sub: message.sub,
                dub: message.dub
            }
        }

        CachedValues.set(anime.id.toString(), anime);
        episodesReady = true;
    })

    const handleScroll = (e: Event) => {
        scrollY = (e.target as HTMLElement).scrollTop;
    }

    const dragStart = (e: TouchEvent | MouseEvent) => {
        if (scrollY > 0) return;
        // if (isScrolling) return;
        
        if (e instanceof MouseEvent) {
            positionDiff = e.pageY;
        } else if (e instanceof TouchEvent) {
            positionDiff = e.touches[0].pageY;
        }
        console.log(positionDiff)
        isDragStart = true;
        previousScroll = wrapper.scrollTop;
    }

    const dragging = (e: TouchEvent | MouseEvent) => {
        if (!isDragStart) return;
        e.preventDefault();
        isDragging = true;
        let diff: number;
        if (e instanceof MouseEvent) {
            diff = e.pageY - positionDiff;
        } else if (e instanceof TouchEvent) {
            diff = e.touches[0].pageY - positionDiff;
        } else {
            return;
        }
        if (isTryingToHide && margin <= screenY/8 && margin >= 0) {
            margin = diff;
            return;
        }

        if (isScrolling) {
            wrapper.scrollTop = previousScroll - diff;
        }

        if (diff < 0 && !isScrolling && !isTryingToHide) {
            isScrolling = true;
        } 
        if (diff > 0 && !isScrolling && !isTryingToHide) {
            isTryingToHide = true;
        }
    }

    const dragStop = () => {
        isDragStart = false;
        if (isScrolling) {
            isScrolling = false;
        }
        if (isTryingToHide) {
            isTryingToHide = false;
        }
        if (!isDragging) return
        isDragging = false;
        if (margin >= screenY/10) {
            History.pop();
        } else {
            while (margin > 0) {
                margin -= 1;
            }
            margin = 0;
        }
    }
</script>

<svelte:document on:mouseup={dragStop} on:touchend={dragStop} on:mousemove={dragging}/>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<section bind:clientWidth={wrapperWidth} class="anime-info-wrapper {loading ? 'hide' : 'show'}" style="margin-top: {margin}px;" bind:this={wrapper} bind:clientHeight={screenY} on:mousedown={dragStart} on:touchstart={dragStart} on:touchmove={dragging} on:scroll={handleScroll}>
    {#if anime}
        <div class="player-section">
            <button class="watch-now">
                Watch now
            </button>
            <button class="watch-together">
                <WatchTogether class="watch-together-svg"/>
            </button>
            <button class="bookmark">
                <Bookmark class="bookmark-svg"/>
            </button>
        </div>
        <span class="anime-banner" style={anime.bannerImage ? `background-image: url(${anime.bannerImage});` : `background-color: ${anime.coverImage.color};`} />
        <div class="cover">
            <span class="anime-cover" style="background-image: url({anime.coverImage.extraLarge ?? anime.coverImage.large});"/>
        </div>
        <h1 style="font-size: {wrapperWidth * 0.05}px;">{title}</h1>
        <AnimeGenres genres={anime.genres}/>
        <AnimeDescription description={anime.description}/>
        <AnimeEpisodes anime={anime.gogoanime} episodesReady/>
    {:else}
    <div class="loading">
        <div class="loader"/>
    </div>
    {/if}
    <!-- <FillerSpace height="100vh"/> -->
</section>

<style>
    section {
        width: 100vw;
        height: 100vh;
        background-color: var(--background-secondary);
        border-radius: 0;
        box-shadow: 0 -0.75rem 1rem 1rem rgba(0, 0, 0, 0.5);
        overflow-x: hidden;
        
        position: relative;
    }
    section.hide {
        overflow-y: hidden;
    }
    section.show {
        overflow-y: scroll;
    }
    /* :global(.anime-info-wrapper .player-section .watch-now svg) {
        fill: var(--primary);
        height: 60%;
        width: 60%;

    } */
    .player-section {
        position: fixed;
        bottom: 0;
        left: 0;
        width: 100%;
        height: 6rem;
        padding: 1rem;
        padding-bottom: 2rem;
        box-sizing: border-box;
        
        display: flex;
        justify-content: space-between;
        gap: 1rem;
        z-index: 1;
    }
    .player-section::before {
        content: "";
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
        z-index: -1;
        background: linear-gradient(180deg, rgba(0, 0, 0, 0) 0%, rgba(24, 24, 24, 0.5)50%, rgb(24, 24, 24)  100%);
    }
    .player-section button {
        height: 100%;
        aspect-ratio: 1;
        background: var(--background-secondary);
        border-radius: 0.25rem;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .player-section .watch-now {
        width: 100%;
        border: none;
        background-color: var(--primary);
        color: var(--background);
        font-size: 1.5rem;
    }


    .player-section .watch-together {
        border-color: var(--primary);
    }

    :global(section .player-section .watch-together svg) {
        fill: var(--primary);
        height: 60%;
        width: 60%;
    }

    :global(section .player-section .bookmark svg) {
        fill: var(--primary);
        height: 60%;
        width: 60%;
    }

    .player-section .bookmark {
        border-color: var(--primary);
    }

    h1 {
        width: 80%;
        margin: 0 auto;
        text-align: center;
        margin-top: 1rem;
        animation: swing-in-top-bck 0.75s cubic-bezier(0.175, 0.885, 0.320, 1.275) both;
        animation-delay: 0.5s;
    }

    .cover {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: 10rem;
        position: relative;
        margin-top: 10rem;
        animation: slide-in-bck-center 0.7s cubic-bezier(0.250, 0.460, 0.450, 0.940) both;
        animation-delay: 0.25s;
    }

    .cover .anime-cover {
        display: block;
        height: 100%;
        aspect-ratio: 13/19;
        background-size: cover;
        background-position: center;
        border-radius: 1rem;
        box-shadow: 0 0.1rem 0.5rem 0.5rem rgba(0, 0, 0, 0.25);
    }

    .anime-banner {
        display: block;
        width: 100%;
        aspect-ratio: 1;
        background-size: cover;
        background-position: center;
        position: absolute;
    }

    .anime-banner::after {
        content: "";
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
        background: linear-gradient(180deg, rgba(0,0,0,0.5) 0%,  var(--background-secondary) 100%);
    }

    .loading {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    .loader {
        width: 50px;
        aspect-ratio: 1;
        border-radius: 50%;
        background: 
            radial-gradient(farthest-side,#ffa516 94%,#0000) top/8px 8px no-repeat,
            conic-gradient(#0000 30%,#ffa516);
        mask: radial-gradient(farthest-side,#0000 calc(100% - 8px),#000 0);
        animation: l13 1s infinite linear;
    }
    @keyframes l13{ 
        100%{transform: rotate(1turn)}
    }

    @keyframes slide-in-bck-center{0%{transform:translateZ(600px);opacity:0}100%{transform:translateZ(0);opacity:1}}
    @keyframes swing-in-top-bck{0%{transform:rotateX(70deg);transform-origin:top;opacity:0}100%{transform:rotateX(0deg);transform-origin:top;opacity:1}}
</style>
