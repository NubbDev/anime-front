<script lang="ts">
    import type { AnimeCardInfo } from "$lib";

    export let anime: AnimeCardInfo;
</script>

<div>
    <button style="background-image: url({anime.coverImage.large}); background-color: {anime.coverImage.color}">

    </button>
    <h4>{anime.title.english}</h4>
</div>




<style>
    button {
        width: 30vw;
        height: 20vh;
        background-size: cover;
        background-position: center;
        border: none;
        border-radius: 0.5rem;
    }
</style>