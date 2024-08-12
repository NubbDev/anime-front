<script lang="ts">
    import type { AnimeCardInfo } from "$lib";

    export let anime: AnimeCardInfo;
</script>

<button style="background-image: {anime.coverImage.large}; background-color: {anime.coverImage.color}">


</button>

<style>
    button {
        width: 30vw;
    }
</style>