<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
    export let anime: AnimeCardInfo;

    const genreText = anime.genres.join(", ");
    console.log(genreText);
</script>

<section style="background-image: url({anime.bannerImage});">
    <div class="overlay"></div>
    <div class="hero-text">
        <h3>{anime.title.english}</h3>
        <p>{genreText}</p>
    </div>
    <!-- <button>Watch Now</button>
    <button>Add to Watchlist</button>
    <button>More Info</button> -->
</section>

<style>
    section {
        width: 100%;
        height: 30vh;
        top: 0;
        left: 0;
        
        background-size: cover;
        background-position: center;
        position: relative;
    }

    

    p {
        color: white;
        text-shadow: 2px 2px 2px black;
    }
    section .overlay {
        background: linear-gradient(180deg, rgba(0, 0, 0, 0.5) , var(--background));
        width: 100%;
        height: 100.1%;
        position: absolute;
        z-index: 0;
    }

    section .hero-text {
        position: absolute;
        z-index: 1;
        top: 75%;
        left: 5vw;
        transform: translate(0, -50%);
        text-align: left;
        text-shadow: 2px 2px 2px black;
    }
    section .hero-text h3 {
        font-size: 5vw;
        margin: 0;
        color: white;
    }
    section .hero-text p {
        font-size: 3vw;
        margin: 0 0;
        color: var(--sub-text);
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