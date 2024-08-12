<script lang="ts">
  import type { AnimeCardInfo } from "$lib";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
    export let command_name: string;
    export let animes: AnimeCardInfo[] = [];
    export let title: string;

    onMount(async () => {
        const data: AnimeCardInfo[] = await invoke(command_name, { page: "" });
        animes = data;
    });

</script>

{#if animes.length > 0}
    <h3>{title}</h3>
    {#each animes as anime}
        <div>
            <img src={anime.coverImage.large} alt={anime.title.english} />
        </div>
    {/each}
{/if}
