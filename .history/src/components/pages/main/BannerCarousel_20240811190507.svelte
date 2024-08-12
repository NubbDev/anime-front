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
                <img src={anime.bannerImage} alt={anime.title.english} />
                <div class="carousel-caption">
                    <h3>{anime.title.english}</h3>
                    <p>{anime.genres.join(", ")}</p>
                </div>
            </div>
        {/each}
    </div>
</section>
