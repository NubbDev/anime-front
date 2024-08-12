<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
  import type { MouseEventHandler, TouchEventHandler } from "svelte/elements";
    export let anime: AnimeCardInfo[];

    let currentSlide = 0;
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
        carousel.classList.add("dragging");
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
        if (!isDragging) return
        isDragging = false;
        adjustSlide();
    }

    const adjustSlide = () => {
        console.log("adjusting slide");
        if (carousel.scrollLeft - (carousel.scrollWidth - carousel.clientWidth) > -1 || carousel.scrollLeft <= 0) return;
        positionDiff = Math.abs(positionDiff);
        let valDiff = (firstImgWidth) - positionDiff;
        if (carousel.scrollLeft > prevScollLeft) {
            carousel.scrollLeft += positionDiff > firstImgWidth / 3 ? valDiff : -positionDiff;
        } else {
            carousel.scrollLeft -= positionDiff > firstImgWidth / 3 ? valDiff : -positionDiff;
        }
    }

    
</script>

<svelte:document on:mouseup={dragStop} on:touchend={dragStop} on:mousemove={dragging}/>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<section class="wrapper">
    <div class="carousel" bind:this={carousel} on:mousedown={dragStart} on:touchstart={dragStart} on:touchmove={dragging}>
        {#each anime as anime, i}
            <div class="carousel-item {i === currentSlide ? 'active' : ''}" bind:clientWidth={firstImgWidth}>
                <div class="img" style="background-image: url({anime.bannerImage});"></div>
                <div class="carousel-caption">
                    <h3>{anime.title.english}</h3>
                    <p>{anime.genres.join(", ")}</p>
                </div>
            </div>
        {/each}
    </div>
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
        overflow: scroll;
        scroll-behavior: smooth;
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
    }
    
    .carousel.dragging .carousel-item .img {
        pointer-events: none;
    }

    .carousel .carousel-item .img:first-child {
        margin-left: 0px;
    }   


</style>
