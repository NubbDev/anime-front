<script lang="ts">
    import type { AnimeCardInfo } from "$lib";

    export let anime: AnimeCardInfo;

    let animeCover = anime.coverImage.large;
    let animeColor = anime.coverImage.color;
    let animeTitle = anime.title.english;

    if (animeTitle.length > 20) {
        animeTitle = animeTitle.slice(0, 20) + "...";
    }
</script>

<div>
    <span>{animeTitle}</span>
    <button style="background-image: url({animeCover}); background-color: {animeColor}">

    </button>
</div>




<style>

    div {
        /* display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center; */
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
    span {
        font-size: 3vw;
        color: var(--headings);
        margin: 0;
        writing-mode: vertical-rl;
    }
</style>