<script lang="ts">
  import type { AnimeCardInfo } from "$lib";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
    export let command_name: string;
    export let title: string;
    
    let animes: AnimeCardInfo[] = [];
    onMount(async () => {
        const data: AnimeCardInfo[] = await invoke(command_name, { page: "" });
        animes = data;
    });

</script>


{#if animes.length > 0}
    <section>
        <h3>{title}</h3>
        <div>
            {#each animes as anime}
            <button>
                <img src={anime.coverImage.large} alt={anime.title.english} />
                <!-- <p>{anime.title.english}</p> -->
            </button>
            {/each}
            </div>
    </section>
{/if}

<style>
    section {
        /* display: flex; */
        flex-direction: column;
        align-items: flex-start;
        width: 100vw;
    }

    h3 {
        font-size: 1.5rem;
        margin: 1rem 0;
    }

    div {
        display: inline-block;
        overflow: auto;
        white-space: nowrap;
    }

    button {
        display: inline-block;
        margin: 0 1rem;
        border: none;
        background: none;
        cursor: pointer;
    }

    img {
        width: 30vw;
        border-radius: 10px;
    }
</style>