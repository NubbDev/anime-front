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
</div>




<style>

    div {
        position: relative;
        margin: 0 0.5rem;
    }
    button {
        position: relative;
        width: 35vw;
        height: 20vh;
        
        border: none;
        border-radius: 0.5rem;
        cursor: pointer;
    }
    button span {
        background-size: cover;
        background-position: center;
        border-radius: 0.5rem;
        width: 30vw;
        height: 20vh;
        position: absolute;
        top: 0;
        left: 0;
    }
    button p {
        font-size: 3vw;
        margin: 0;
        position: relative;
        top: 0;
        left: 29.5vw;
        writing-mode: vertical-lr;
        text-align: center;
    }

    /* button:hover span {
        width: vw;
    } */
</style>