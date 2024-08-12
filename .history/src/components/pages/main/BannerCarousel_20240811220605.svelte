<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
  import { onMount } from "svelte";
  import type { MouseEventHandler, TouchEventHandler } from "svelte/elements";
  import { get, writable } from "svelte/store";
    export let anime: AnimeCardInfo[];

    let currentSlide = writable(0);
    let previousSlide = 0;
    let slideChange = 0;
    let slides = [];

    let isDragStart = false, isDragging = false, prevPageX: number, prevScollLeft: number, positionDiff: number;

    let carousel: HTMLDivElement;

    let firstImgWidth: number;


    const dragStart = (e: TouchEvent | MouseEvent) => {
        isDragStart = true;
        if (e instanceof MouseEvent) {
            prevPageX = e.pageX;
        } else if (e instanceof TouchEvent) {
            prevPageX = e.touches[0].pageX;
        }
        prevScollLeft = carousel.scrollLeft;
    }

    const dragging = (e: TouchEvent | MouseEvent) => {
        if (!isDragStart) return;
        e.preventDefault();
        carousel.style.scrollBehavior = "auto";
        isDragging = true;
        if (e instanceof MouseEvent) {
            positionDiff = e.pageX - prevPageX;
        } else if (e instanceof TouchEvent) {
            positionDiff = e.touches[0].pageX - prevPageX;
        }
        carousel.scrollLeft = prevScollLeft - positionDiff;
        slideChange = (Math.floor(carousel.scrollLeft / firstImgWidth));
    }

    const dragStop = () => {
        isDragStart = false;
        carousel.classList.remove("dragging");
        carousel.style.scrollBehavior = "smooth";
        if (!isDragging) return
        isDragging = false;
        adjustSlide();
    }

    const adjustSlide = () => {
        let current = get(currentSlide);
        previousSlide = current;
        if (positionDiff > firstImgWidth / 3) { // if the user has dragged more than 1/3 of the image width to the left
            currentSlide.set(slideChange);
        } else if (positionDiff < -firstImgWidth / 3) { // if the user has dragged more than 1/3 of the image width to the right
            currentSlide.set(previousSlide + 1);
        } else {
            carousel.scrollLeft = firstImgWidth * previousSlide;
        } 
    }

    currentSlide.subscribe(value => {
        if (carousel) {
            return carousel.scrollLeft = firstImgWidth * value;
        } 
    })


    
</script>

<svelte:document on:mouseup={dragStop} on:touchend={dragStop} on:mousemove={dragging}/>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<section class="wrapper">
    <div class="carousel" bind:this={carousel} style="scroll-behavior: smooth;" on:mousedown={dragStart} on:touchstart={dragStart} on:touchmove={dragging}>
        {#each anime as anime, i}
            <div class="carousel-item {i === get(currentSlide) ? 'active' : ''}" bind:clientWidth={firstImgWidth}>
                <div class="img" style="background-image: url({anime.bannerImage});"></div>
                <div class="carousel-caption">
                    <h3>{anime.title.english}</h3>
                    <p>{anime.genres.join(", ")}</p>
                </div>
            </div>
        {/each}
    </div>
    {#if anime.length > 1}
        <div class="indicators">
            {#each anime as _, i}
                <button class="indicator {i === get(currentSlide) ? 'active' : ''}" on:click={() => currentSlide.set(i)}></button>
            {/each}
        </div>
    {/if}
</section>

<style>
    .wrapper {
        width: 100vw;
        height: 35vh;
        position: relative;
        display: flex;
    }
    
    .wrapper .carousel {
        width: 100vw;
        height: 35vh;
        display: flex;
        transition: transform 0.5s;
        white-space: nowrap;
        overflow: hidden;
    }

    .carousel::after {
        content: "";
        width: 100vw;
        height: 35vh;
        position: absolute;
        top: 0;
        left: 0;
        background: linear-gradient(180deg, rgba(0, 0, 0, 0.5), var(--background));
    }

    .wrapper .carousel .carousel-item {
        width: 100vw;
        height: 35vh;
        position: relative;
        display: inline-block;
        text-shadow: 2px 2px 2px black;
    }

    .wrapper .carousel .carousel-item .img {
        width: 100vw;
        height: 35vh;
        background-size: cover;
        background-position: center;
    }   

    .wrapper .carousel .carousel-item .carousel-caption {
        position: absolute;
        z-index: 1;
        top: 75%;
        left: 5vw;
        transform: translate(0, -50%);
        text-align: left;
        text-shadow: 2px 2px 2px black;
        font-size: 2vw;
    }

    .carousel .carousel-item .img:first-child {
        margin-left: 0px;
    }   
    .indicator {
        width: 0.5rem;
        height: 0.1rem;
        /* border-radius: 100%; */
        background-color: white;
        margin: 0 0.1rem;
        cursor: pointer;
        border: none;
    }

    .indicator.active {
        background-color: var(--primary);
        height: 0.5rem;
    }

    .indicators {
        position: absolute;
        bottom: 0.5rem;
        left: 50%;
        transform: translate(-50%, 0);
    }


</style>
