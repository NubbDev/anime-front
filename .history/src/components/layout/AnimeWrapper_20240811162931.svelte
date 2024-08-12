<script lang="ts">
    import type { AnimeCardInfo } from "$lib";

    export let anime: AnimeCardInfo;
</script>

<div>
    <h4>{anime.title.english}</h4>
    <button style="background-image: url({anime.coverImage.large}); background-color: {anime.coverImage.color}">

    </button>
</div>




<style>

    div {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        margin: 0 1rem;
    }
    button {
        width: 30vw;
        height: 20vh;
        background-size: cover;
        background-position: center;
        border: none;
        border-radius: 0.5rem;
    }
    h4 {
        font-size: 3vw;
        color: var(--headings);
        margin: 0;
        transform: rotateX('90');
    }
</style>