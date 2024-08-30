<script lang="ts">
    import { clipTitle, type AnimeGenres } from "$lib";
    import { Image } from "@unpic/svelte";

    export let banner: string | undefined;
    export let title: string;
    export let genres: (AnimeGenres)[]

    let height = 0;
    let width = 0;
</script>

{#if banner}
    <div class="img" bind:clientHeight={height} bind:clientWidth={width}>
        <Image src={banner} width={width} height={height}/>
    </div>
{:else}
    <div class="no_banner"></div>
{/if}
<div class="carousel-caption">
    <h3>{`${clipTitle(title, 25)}`}</h3>
    <p>{genres.join(", ")}</p>
</div>
<style>
    h3 {
        font-size: 5vw;
        margin: 0;
        width: 55vw;
        text-wrap: wrap;
        color: var(--headings);
    }

    p {
        font-size: 3vw;
        color: var(--sub-text);
        margin-top: 0.1rem;
    }

    .img {
        width: 100vw;
        height: 35vh;

        background-position: top center;
        background-attachment: fixed;
        background-repeat: no-repeat;
        background-size: auto 40%;
    }   

    .carousel-caption {
        position: absolute;
        z-index: 1;
        top: 75%;
        left: 5vw;
        transform: translate(0, -50%);
        text-align: left;
        text-shadow: 2px 2px 2px black;
        font-size: 2vw;
    }

    .no_banner {
        width: 100vw;
        height: 35vh;
    }
</style>