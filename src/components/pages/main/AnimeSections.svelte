<script lang="ts">
    import { type AnimeCardInfo, PageIndex, PageStore } from "$lib";
    import AnimeWrapper from "../../layout/AnimeWrapper.svelte";
    export let title: string;
    export let animes: AnimeCardInfo[] = [];

    let buttonPage: PageIndex;
    switch(title.toLowerCase()) {
        case "trending":
            buttonPage = PageIndex.TRENDING;
            break;
        case "popular":
            buttonPage = PageIndex.POPULAR;
            break;
        case "top":
            buttonPage = PageIndex.TOP;
            break;
    }
    

</script>


{#if animes.length > 0}
    <section>
        <span>
            <h3>{title}</h3>
            <button on:click={() => PageStore.set(buttonPage)}>See all</button>
        </span>
        <div>
            {#each animes as anime}
                <AnimeWrapper {anime}/>
            {/each}
        </div>
    </section>
{/if}

<style>
    section {
        width: 100vw;
        margin: 1rem 0;
    }

    h3 {
        font-size: 1.5rem;
        margin: 0.25rem 0;
        /* width: vw; */
    }

    button {
        font-size: 1rem;
        color: var(--primary);
        cursor: pointer;

        background: none;
        border: none;
        padding: 0;
    }

    div {
        /* display: inline-block; */
        overflow: auto;
        white-space: nowrap;
        display: flex;
    }

    span {
        display: flex;
        justify-content: space-between;
        margin: 0 1rem;
    }


</style>