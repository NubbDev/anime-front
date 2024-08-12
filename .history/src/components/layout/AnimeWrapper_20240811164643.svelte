<script lang="ts">
    import type { AnimeCardInfo } from "$lib";

    export let anime: AnimeCardInfo;

    let animeCover = anime.coverImage.large;
    let animeColor = anime.coverImage.color;
    let animeTitle = anime.title.english;

    if (animeTitle.length > 25) {
        let splitTitle = animeTitle.split(" ");
        animeTitle = "";
        while (animeTitle.length < 20) {
            animeTitle += splitTitle.shift() + " ";
        }
        animeTitle += "...";
    }
</script>

<div>
    <button style="background-image: url({animeCover}); background-color: {animeColor}">
        
    </button>
    
    <span style="background-color: {animeColor}">
        <p>{animeTitle}</p>
    </span>
</div>




<style>

    div {
        position: relative;
        /* display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center; */
        margin-right: 1rem;
    }
    button {
        position: relative;
        width: 30vw;
        height: 20vh;
        background-size: cover;
        background-position: center;
        border: none;
        border-radius: 0.5rem 0 0 0.5rem;
        z-index: 1;
    }
    span {
        font-size: 3.25vw;
        color: var(--headings);
        margin: 0;
        writing-mode: vertical-lr;
        position: relative;
        left: -5px;
        height: 20vh;
        z-index: 0;
        border-radius: 0 0.5rem 0.5rem 0;
        
    }
</style>