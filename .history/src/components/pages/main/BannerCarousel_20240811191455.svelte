<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
    export let anime: AnimeCardInfo[];

    let currentSlide = 0;
    let slides = [];

    
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
        position: relative;
        display: flex;
    }


</style>
