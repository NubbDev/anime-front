<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
    export let anime: AnimeCardInfo;
</script>

<section>
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
        background-image: url({anime.bannerImage});
        background-size: cover;
        background-position: center;
        position: absolute;
    }
</style>