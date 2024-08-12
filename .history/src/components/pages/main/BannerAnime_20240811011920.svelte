<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
    export let anime: AnimeCardInfo;

    const genreText = anime.genres.map(genre => genre).join(", ");
</script>

<section style="background-image: url({anime.bannerImage});">
    <h3>{anime.title.english}</h3>
    <p>{anime.description}</p>
    <button>Watch Now</button>
    <button>Add to Watchlist</button>
    <button>More Info</button>
</section>

<style>
    section {
        width: 100vw;
        height: 25vh;
        
        background-size: cover;
        background-position: center;
        position: absolute;
    }

    h3 {
        font-size: 2rem;
        margin: 0 0;
        color: white;
        text-shadow: 2px 2px 2px black;
    }

    p {
        color: white;
        text-shadow: 2px 2px 2px black;
    }

    button {
        margin: 1rem;
        padding: 0.5rem 1rem;
        border: none;
        background: white;
        color: black;
        cursor: pointer;
    }
</style>