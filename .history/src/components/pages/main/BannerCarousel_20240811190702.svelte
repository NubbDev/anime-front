<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
    export let anime: AnimeCardInfo[];

    let currentSlide = 0;
    let slides: HTMLCollection;
</script>

<section class="carousel">
    <div class="carousel-inner">
        {#each anime as anime, i}
            <div class="carousel-item {i === currentSlide ? 'active' : ''}">
                <div class="img" style="background-image: url({anime.bannerImage});"></div>
                <div class="carousel-caption">
                    <h3>{anime.title.english}</h3>
                    <p>{anime.genres.join(", ")}</p>
                </div>
            </div>
        {/each}
    </div>
</section>

<style>
    .carousel {
        width: 100%;
        height: 35vh;
        overflow: hidden;
        position: relative;
    }

    .carousel-inner {
        display: flex;
        transition: transform 0.5s;
    }

    .carousel-item {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        background-size: cover;
        background-position: center;
        position: relative;
    }

    .carousel-caption {
        position: absolute;
        z-index: 1;
        top: 75%;
        left: 5vw;
        transform: translate(0, -50%);
        text-align: left;
        text-shadow: 2px 2px 2px black;
    }

    .carousel-caption h3 {
        font-size: 5vw;
        margin: 0;
        color: white;
    }

    .carousel-caption p {
        color: white;
        text-shadow: 2px 2px 2px black;
    }

    .img {
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
        background-size: cover;
        background-position: center;
    }

    .active {
        transform: translateX(0);
    }

    .carousel-item {
        transform: translateX(-100%);
    }

    .carousel-item.active {
        transform: translateX(0);
    }

    .carousel-item.next {
        transform: translateX(100%);
    }

    .carousel-item.prev {
        transform: translateX(-100%);
    }

    .carousel-item {
        transition: transform 0.5s;
    }
</style>
