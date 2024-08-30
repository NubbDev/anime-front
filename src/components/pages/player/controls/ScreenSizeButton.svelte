<script lang="ts">
    import FullScreenIcon from "$lib/assets/icons/expand.svg?component";
    import FullScreenExitIcon from "$lib/assets/icons/compress.svg?component";
    export let fullScreen: boolean;
    export let onClick = () => {};

    let handleClick = () => {
        fullScreen = !fullScreen;
        onClick();
    }
</script>

<button on:click={handleClick} class="player-fullscreen-button"> 
    <FullScreenIcon class={fullScreen ? 'hide' : 'show'}/>
    <FullScreenExitIcon class={!fullScreen ? 'hide' : 'show'}/>
</button>

<style>
    button {
        width: 7vw;
        height: 7vw;
        background-color: transparent;
        border: none;
        padding: 0;
        position: relative;
    }

    :global(.player-fullscreen-button svg) {
        width: 100%;
        height: 100%;
        transition: opacity 0.25s ease;
        fill: var(--headings);
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }

    :global(.player-fullscreen-button svg.hide) {
        opacity: 0;
    }

    :global(.player-fullscreen-button svg.show) {
        opacity: 1;
    }
</style>