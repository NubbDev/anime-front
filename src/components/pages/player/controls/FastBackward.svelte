<script lang="ts">
    export let onClick: () => void;
    let isDoubleClicked = false;
    let running = false;
    
    const handleClick = (event: MouseEvent) => {
        if (!isDoubleClicked) {
            isDoubleClicked = true;
            return setTimeout(() => {
                isDoubleClicked = false;
            }, 300);
        }
        running = true;
        onClick();
        isDoubleClicked = false;
        setTimeout(() => {
            running = false;
        }, 300);
    }
</script>
<button on:click={handleClick} class="{running ? 'show' : ''}">
    <span/>
</button>

<style>
    button {
        height: 100%;
        aspect-ratio: 1;
        background-color: transparent;
        border: none;
        padding: 0;
        position: relative;
        transition: background 1s ease-in-out;
    }

    button span {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: linear-gradient(-90deg, rgba(0, 0, 0, 0) 0%, rgba(255, 255, 255, 0.25) 100%);
        opacity: 0;
        transition: opacity 0.25s ease-in-out;
    }

    button.show span {
        opacity: 1;
    }
    
</style>