<script lang="ts">
    import { type AnimeCardInfo } from "$lib";
    import { onMount } from "svelte";

    export let anime: AnimeCardInfo;
    let animeTitle = anime.title.english || anime.title.romaji;
    export let display: 'show' | 'hide' | 'hide-back' = 'hide';
    onMount(() => {

        if (animeTitle.length > 25) {
            let splitTitle = animeTitle.split(" ");
            animeTitle = "";
            while (animeTitle.length < 20) {
                animeTitle += splitTitle.shift() + " ";
            }
            animeTitle += "...";
        }
    });

    
</script>

<button class="{display}">
    <span style="background-image: url({anime.coverImage?.medium});"></span>
    <div>
        <h2>
            {animeTitle}
        </h2>
    </div>
</button>

<style>
    button {
        width: 100%;
        height: 5rem;
        background: none;
        border: none;
        display: flex;
        gap: 0.75rem;
        transition: transform 0.5s ease, opacity 0.5s ease;
    }

    button.show {
        transform: translateX(0);
        opacity: 1;
    }

    button.hide {
        transform: translateX(calc(100vw + 110%));
        opacity: 0;
    }

    button.hide-back {
        transform: translateX(calc((100vw + 110%) * -1));
        opacity: 0;
    }

    button span {
        height: 100%;
        width: 5rem;
        background-size: cover;
        border-radius: 0.5rem;
    }
    button div {
        width: 100%;
        height: 100%;
        border-radius: 0.5rem;
        display: flex;
        flex-direction: column;
        background-color: #00000080;
        padding: 0.5rem;
        box-sizing: border-box;
    }

    button div h2 {
        font-size: 1rem;
        margin: 0.5rem;
        text-align: left;
        color: var(--headings);
    }
</style>