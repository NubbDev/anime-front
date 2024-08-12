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
            <button>See all</button>
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