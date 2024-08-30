<script lang="ts">
    import { History, type AnimeCardInfo, AnimeSelectedStore, CachedValues, ClientWSMessageType, clipTitle, MediaFormat, PageIndex, TimeElapsedStore, Websocket, type AnimeInfo} from "$lib";
    import { onMount } from "svelte";
    import { Image } from '@unpic/svelte'

    export let anime: AnimeCardInfo;
    export let height: number = 0;
    export let delay: number = 0;
    let animeCover = anime.coverImage?.large ?? anime.coverImage?.medium!;
    let animeColor = anime.coverImage?.color;
    let animeTitle = clipTitle(anime.title.english ?? anime.title.romaji, 25);
    let format: MediaFormat | string | undefined = anime.format;
    let formatColor = anime.format;
    let picture_height = 0;
    let picture_width = 0;

    onMount(() => {

        if (format) {

            format = format.replace("_", " ");
        }
    });


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

    let textColor = getTextColor(animeColor as string);

    const setAnime = async () => {
        const cached = await CachedValues.get<AnimeInfo>(anime.id.toString());
        if (cached) {
            const date = new Date();
            if (date.getTime() - cached.lastUpdated.getTime() > 1000 * 60 * 10) {
                await Websocket.send(ClientWSMessageType.GetAnime, { id: anime.id });
                return;
            }
            AnimeSelectedStore.set(cached.data);
            History.push(PageIndex.ANIME);
            return;
        }
        await Websocket.send(ClientWSMessageType.GetAnime, { id: anime.id });
    }
</script>

<div bind:clientHeight={height} style="animation-delay: {delay * 0.1}s;">
    <button style="background-color: {animeColor}" on:click={setAnime}>
        <span class="media_format {formatColor}" >{format}</span>
        <span class="anime-pic" bind:clientHeight={picture_height} bind:clientWidth={picture_width}>
            <Image src="{animeCover}" layout="constrained" height={picture_height} width={picture_width} style="border-radius: 0.5rem;"/>
        </span>
        <p style="color: {textColor};">{animeTitle}</p>
    </button>
</div>




<style>

    div {
        opacity: 0;
        position: relative;
        margin: 0 0.5rem;
        animation-name: fadeIn;
        animation-duration: 0.5s;
        animation-fill-mode: forwards;

    }
    button {
        position: relative;
        width: clamp(8rem, 33vw, 15rem);
        aspect-ratio: 13/19;

        border: none;
        border-radius: 0.5rem;
        cursor: pointer;
    }
    button .media_format {
        position: absolute;
        z-index: 1;
        bottom: 0.5rem;
        left: 0.5rem;
        padding: 0.25rem 0.5rem;
        border-radius: 0.5rem;
        font-weight: 700;
        color: var(--background);
    }

    button .anime-pic {
        display: block;
        background-size: cover;
        background-position: center;
        border-radius: 0.5rem;
        width: calc(100% - clamp(0.5rem, 3.5vw, 2.5rem));
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
        
    }

    button p {
        font-size: clamp(0.75rem, 2.25vw, 3rem);
        margin: 0;
        position: relative;
        top: 0;
        left: calc(100% - clamp(0.5rem, 2.25vw, 2rem));
        writing-mode: vertical-lr;
        text-align: center;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    .TV {
        background-color: #fea3aa ;
    }

    .TV_SHORT {
        background-color: #f8b88b ;
    }

    .MOVIE {
        background-color: #faf884 ;
    }

    .SPECIAL {
        background-color: #baed91;
    }

    .OVA {
        background-color: #b2cefe ;
    }

    .ONA {
        background-color: #f2a2e8 ;
    }

    .MUSIC {
        background-color: #b2fefe;
    }



    /* button:hover span {
        width: vw;
    } */
</style>