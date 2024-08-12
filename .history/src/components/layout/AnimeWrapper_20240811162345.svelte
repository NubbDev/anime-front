<script lang="ts">
    import type { AnimeCardInfo } from "$lib";

    export let anime: AnimeCardInfo;
</script>

<button style="background-image: url({anime.coverImage.large}); background-color: {anime.coverImage.color}">


</button>

<style>
    button {
        width: 30vw;
        height: 20vh;
        background-size: cover;
        background-position: center;
        border: none;
        border-radius: 1rem;
    }
</style>