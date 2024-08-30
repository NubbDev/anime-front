<script lang="ts">
    import { onMount } from "svelte";
    import PlayButton from "./controls/PlayButton.svelte";
    import ScreenSizeButton from "./controls/ScreenSizeButton.svelte";
    import VideoSettings from "./controls/VideoSettings.svelte";
    import VolumeButton from "./controls/VolumeButton.svelte";
    import FastBackward from "./controls/FastBackward.svelte";
    import FastForward from "./controls/FastForward.svelte";
    import PictureInPicture from "./controls/PictureInPicture.svelte";
    import {  } from "@tauri-apps/api/window";


    let video: HTMLVideoElement;
    let playing: boolean;
    let videoFullScreen: boolean = false
    onMount(() => {
        playing = !video.paused;
        
    });

    const handlePlayButton = () => {
        playing ? video.play() : video.pause();
    }

    const handleFastForward = () => {
        if (video.currentTime > video.duration - 10) {
            video.currentTime = video.duration;
            return;
        }
        video.currentTime += 10;
    }

    const handleFastBackward = () => {
        if (video.currentTime < 10) {
            video.currentTime = 0;
            return;
        }
        video.currentTime -= 10;
    }

    const handleFullScreen = async () => {
        if (videoFullScreen) {
            window
            // await video.requestFullscreen();
        } else {
            // document.exitFullscreen();
        }
    }

</script>

<main class={playing ? '' : 'paused'}>
    <section class="video-controls">
        <div>
            <span>
                <button>back</button>
                <h6>Anime Title</h6>
            </span>
            <span>
                <VideoSettings />
                <PictureInPicture />
                <ScreenSizeButton bind:fullScreen={videoFullScreen} onClick={handleFullScreen}/>
            </span>
        </div>
        <div class="main-controls">
            <FastBackward onClick={handleFastBackward}/>
            <PlayButton bind:playing onClick={handlePlayButton}/>
            <FastForward onClick={handleFastForward}/>
        </div>
        <div class="time">
            
            <VolumeButton />
            <span class="timeline"></span>
        </div>
    </section>
    <video controls={false} bind:this={video}>
        <source src="videos/videoExample.mp4"/>
        <track kind="captions">
    </video>
</main>

<style>
    main {
        width: 100vw;
        max-width: 50rem;
        aspect-ratio: 16/9;
        background: var(--background);
        position: relative;
    }

    section {
        position: absolute;
        bottom: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        z-index: 5;
        opacity: 0;
        transition: opacity 0.15s ease-in-out;

        display: grid;
        grid-auto-columns: 1fr;
        grid-template-rows: 0.5fr 2fr 0.5fr;
    }

    section div:nth-child(1) {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 0.75rem;
    }
    section div:nth-child(1) span {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    section div:nth-child(1) span h6 {
        font-size: 1rem;
        margin: 0;
    }

    section div:nth-child(2) {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    section button {
        background: none;
        border: none;
        color: white;
        font-size: 1rem;
        cursor: pointer;
    }

    main:hover section,
    main:focus-within section,
    main.paused section {
        opacity: 1;
    }

    video {
        width: 100%;
        height: 100%;
        object-fit: contain;
    }
</style>