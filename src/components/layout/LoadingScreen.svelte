<script lang="ts">
    import { onMount } from 'svelte';
    import { AppStates, AppStateStore } from '$lib';
    let loaded = false;
    let display: 'hide' | 'show' = 'show';

    AppStateStore.subscribe(value => {
        switch (value) {
            case AppStates.CONNECTING:
                loaded = false;
                setTimeout(() => {
                    display = 'show';
                }, 0.00000001);
                break;
            case AppStates.READY:
                display = 'hide';
                setTimeout(() => {
                    loaded = true;
                }, 1500);
                break;
        }
    })
</script>

{#if !loaded}
    <section class="loading {display}">
        <div class="loader"></div>
        <span/>
    </section>
{/if}

<style>
    section {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        z-index: 1000000000;
        display: flex;
        justify-content: center;
        align-items: center;
        /* transition: opacity 2s ease-out; */
    }

    span {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: -1;
        transition: opacity 3s ease-out, backdrop-filter 1s ease-out, background 1s ease-out;
    }

    div {
    aspect-ratio: 1;
    color: var(--primary);
    background:
        linear-gradient(currentColor 0 0) 100%  0,
        linear-gradient(currentColor 0 0) 0  100%;
    background-size: 50.1% 50.1%;
    background-repeat: no-repeat;
    animation:  l7-0 1s infinite steps(1);
    /* transition: width 2s ease-out; */
    }
    div::before,
    div::after {
    content:"";
    position: absolute;
    inset: 0 50% 50% 0;
    background: currentColor;
    transform: scale(var(--s,1)) perspective(150px) rotateY(0deg);
    transform-origin: bottom right; 
    animation: l7-1 .5s infinite linear alternate;
    }
    div::after {
    --s:-1,-1;
    }
    @keyframes l7-0 {
    0%  {transform: scaleX(1)  rotate(0deg)}
    50% {transform: scaleX(-1) rotate(-90deg)}
    }
    @keyframes l7-1 {
    49.99% {transform:scale(var(--s,1)) perspective(150px) rotateX(-90deg) ;filter:grayscale(0)}
    50%    {transform:scale(var(--s,1)) perspective(150px) rotateX(-90deg) ;filter:grayscale(0.8)}
    100%   {transform:scale(var(--s,1)) perspective(150px) rotateX(-180deg);filter:grayscale(0.8)}
    }

    section.show div {
        width: 6rem;
    }

    section.hide span {
        backdrop-filter: blur(1px);
        background: rgba(0, 0, 0, 0.5);
        opacity: 0;
    }

    section.show span {
        backdrop-filter: blur(100px);
        background: var(--background);
        opacity: 1;
    }
</style>