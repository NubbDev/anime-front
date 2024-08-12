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

    function hexToRgb(hex: string) {
        var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        return result ? {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16)
        } : null;
    }

    function getTextColor(color: string) {
        let rgb = hexToRgb(color);
        if (rgb) {
            let o = Math.round(((rgb.r * 299) +
                (rgb.g * 587) +
                (rgb.b * 114)) / 1000);
            return (o > 125) ? '#000' : '#fff';
        }
        return '#000';
    }

    let textColor = getTextColor(animeColor);

    console.log(textColor);
</script>

<div>
    <button style=" background-color: {animeColor}">
        <span class="anime-pic" style="background-image: url({animeCover});"></span>
        <p style="color: {textColor};">{animeTitle}</p>
    </button>
    
    <span style="background-color: {animeColor}">
    </span>
</div>




<style>

    div {
        position: relative;
        /* display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center; */
        /* margin-right: 1rem; */
    }
    button {
        position: relative;
        width: 30vw;
        height: 20vh;
        
        border: none;
        border-radius: 0.5rem;
        z-index: 1;
        cursor: pointer;
    }
    button span.anime-pic {
        background-size: cover;
        background-position: center;
        width: 100%;
        height: 100%;
    }
    span {
        font-size: 3.25vw;
        /* color: var(--headings); */
        margin: 0;
        writing-mode: vertical-lr;
        position: relative;
        left: -3vw;
        bottom: 0;
        height: 20vh;
        z-index: 0;
        border-radius: 0 0.5rem 0.5rem 0;
    }

    span p {
        writing-mode: vertical-lr;
        text-align: center;
    }
</style>