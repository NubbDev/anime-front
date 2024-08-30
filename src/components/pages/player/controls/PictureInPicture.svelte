<script lang="ts">
    import PictureInPictureIcon from "$lib/assets/icons/images-user.svg?component";
    export let onClick = () => {};
</script>

<button on:click={onClick} class="player-pip-button"> 
    <PictureInPictureIcon />
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

    :global(.player-pip-button svg) {
        width: 100%;
        height: 100%;
        fill: var(--headings);
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }
</style>