<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
  import type { MouseEventHandler, TouchEventHandler } from "svelte/elements";
  import { get, writable } from "svelte/store";
    export let anime: AnimeCardInfo[];

    let currentSlide = writable(0);
    let previousSlide = 0;
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
        // carousel.classList.add("dragging");
        carousel.style.scrollBehavior = "auto";
        isDragging = true;
        if (e instanceof MouseEvent) {
            positionDiff = e.pageX - prevPageX;
        } else if (e instanceof TouchEvent) {
            positionDiff = e.touches[0].pageX - prevPageX;
        }
        carousel.scrollLeft = prevScollLeft - positionDiff;
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
        console.log(positionDiff)
        let diff = Math.abs(positionDiff - firstImgWidth);
        let current = get(currentSlide);
        if (positionDiff > firstImgWidth / 3) {
            if (current == 0) return;
            let distanceToPrev = Math.abs(carousel.scrollLeft - (firstImgWidth * (current - 1)));
            carousel.scrollLeft -= distanceToPrev;
        } else if (positionDiff < -firstImgWidth / 3) {
            if (current === slides.length - 1) return;
            let distanceToNext = Math.abs(carousel.scrollLeft - (firstImgWidth * (current + 1)));
            carousel.scrollLeft += distanceToNext;
        } else {
            console.log("stay")
            let distanceToCurrent = Math.abs(carousel.scrollLeft - (firstImgWidth * current));
            if (positionDiff > 0) {
                carousel.scrollLeft -= distanceToCurrent;
            } else {
                carousel.scrollLeft += distanceToCurrent;
            }
        }

        currentSlide.set(Math.round(carousel.scrollLeft / firstImgWidth));

    }

    currentSlide.subscribe(value => {
        console.log("current slide", value)
        console.log("previous slide", previousSlide)
        if (previousSlide < value) {
            console.log("scroll left")
            carousel.scrollLeft -= firstImgWidth * value;
        } else if (previousSlide > value) {
            console.log("scroll right")
            carousel.scrollLeft += firstImgWidth * value;
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
</section>
<button on:click={() => {previousSlide = get(currentSlide); currentSlide.set(previousSlide - 1);}}>&lt;</button>
<button on:click={() => {previousSlide = get(currentSlide); currentSlide.set(previousSlide + 1);}}>&gt;</button>

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
        /* scroll-behavior: smooth; */
    }

    .wrapper .carousel .carousel-item {
        width: 100vw;
        height: 35vh;
        position: relative;
        display: inline-block;
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

    .carousel.dragging {
        cursor: grabbing;
        scroll-behavior: auto;
    }
    
    .carousel.dragging .carousel-item .img {
        pointer-events: none;
    }

    .carousel .carousel-item .img:first-child {
        margin-left: 0px;
    }   


</style>
