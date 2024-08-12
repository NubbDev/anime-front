<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
  import AnimeWrapper from "../../layout/AnimeWrapper.svelte";
    export let title: string;
    export let animes: AnimeCardInfo[] = [];
    

</script>


{#if animes.length > 0}
    <section>
        <span>
            <h3>{title}</h3>
            <p>See all</p>
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
        margin: 0 1rem;
        width: 90vw;
    }

    div {
        /* display: inline-block; */
        overflow: auto;
        white-space: nowrap;
        display: flex;
    }

    div:first-child() {
        margin-left: 1rem;
    }


</style>