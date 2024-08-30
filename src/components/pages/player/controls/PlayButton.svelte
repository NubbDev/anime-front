<script lang="ts">
    import PlayButton from "$lib/assets/icons/playfilled.svg?component";
    import PauseButton from "$lib/assets/icons/pause.svg?component";
    export let onClick = () => {};

    export let playing: boolean;
    const handleClick = () => {
        playing = !playing;
        onClick();
    }
</script>

<button on:click={handleClick} class="player-play-button"> 
    <PlayButton class={playing ? 'hide' : 'show'}/>
    <PauseButton class={!playing ? 'hide' : 'show'}/>
</button>

<style>
    button {
        width: 20vw;
        height: 100%;
        background-color: transparent;
        border: none;
        padding: 0;
        position: relative;
    }

    :global(.player-play-button svg) {
        width: 100%;
        max-height: 100%;
        transition: opacity 0.25s ease;
        fill: var(--headings);
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }

    :global(.player-play-button svg.hide) {
        opacity: 0;
    }

    :global(.player-play-button svg.show) {
        opacity: 1;
    }
</style>