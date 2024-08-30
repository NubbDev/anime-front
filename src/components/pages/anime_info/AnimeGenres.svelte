<script lang="ts">
  import { AnimeGenres, genreColors } from "$lib";

    export let genres: (AnimeGenres)[];
    let length: number = genres.length;

    //border-color: {genreColors(genre)}; color: {genreColors(genre)};
</script>

<div class="genres">
    {#each genres as genre, i}
        <button style="animation-delay: {Math.abs((genres.length - 1) - i) * 0.2}s; border-color: {genreColors(genre)}; color: {genreColors(genre)};">{genre}</button>
    {/each}
</div>


<style>
    .genres {
        display: flex;
        gap: 0.5rem;
        margin: 1rem 0;
        padding: 0 1rem;
        box-sizing: border-box;
        width: 100%;
        justify-content: center;
        overflow-x: auto;
    }

    button {
        padding: 0.25rem 0.5rem;
        border: 2px solid;
        border-radius: 1rem;
        background-color: transparent;
        font-size: 2.5vw;
        font-weight: 500;
        cursor: pointer;
        animation: slide-in-blurred-left 0.6s cubic-bezier(0.230, 1.000, 0.320, 1.000) both;
    }

    @keyframes slide-in-blurred-left{0%{transform:translateX(-1000px) scaleX(2.5) scaleY(.2);transform-origin:100% 50%;filter:blur(40px);opacity:0}100%{transform:translateX(0) scaleY(1) scaleX(1);transform-origin:50% 50%;filter:blur(0);opacity:1}}
</style>