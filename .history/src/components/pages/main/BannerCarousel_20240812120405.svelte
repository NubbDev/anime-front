<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
    import { get, writable } from "svelte/store";
    export let anime: AnimeCardInfo[];

    let currentSlide = writable(0);
    let previousSlide = 0;
    let slideChange = 0;
    let currentindex = 0; 

    let isDragStart = false, isDragging = false, prevPageX: number, prevScollLeft: number, positionDiff: number;

    let carousel: HTMLDivElement;

    let ImgWidth: number;



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
        slideChange = (Math.floor(carousel.scrollLeft / ImgWidth));
    }

    const dragStop = () => {
        isDragStart = false;
        carousel.style.scrollBehavior = "smooth";
        if (!isDragging) return
        isDragging = false;
        adjustSlide();
    }

    const adjustSlide = () => {
        let current = get(currentSlide);
        previousSlide = current;
        if (positionDiff > ImgWidth / 3) { // if the user has dragged more than 1/3 of the image width to the left
            currentSlide.set(slideChange);
        } else if (positionDiff < -ImgWidth / 3) { // if the user has dragged more than 1/3 of the image width to the right
            currentSlide.set(previousSlide + 1);
        } else {
            carousel.scrollLeft = ImgWidth * previousSlide;
        } 
    }

    currentSlide.subscribe(value => {
        if (value > anime.length - 1) {
            currentindex = value;
            return currentSlide.set(0);
        } else if (value < 0) {
            currentindex = value;
            return currentSlide.set(anime.length - 1);
        }
        
        if (carousel) {
            currentindex = value;
            return carousel.scrollLeft = ImgWidth * value;
        } 
    })


    
</script>

<svelte:document on:mouseup={dragStop} on:touchend={dragStop} on:mousemove={dragging}/>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<section class="wrapper">
    <div class="carousel" bind:this={carousel} style="scroll-behavior: smooth;" on:mousedown={dragStart} on:touchstart={dragStart} on:touchmove={dragging}>
        {#each anime as anime, i}
            <div class="carousel-item {i === get(currentSlide) ? 'active' : ''}" bind:clientWidth={ImgWidth}>
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
                <button class="indicator {i === currentindex ? 'active' : ''}" on:click={() => currentSlide.set(i)}><span/></button>
            {/each}
        </div>
    {/if}
</section>
<section class="more-this-season">
    <button>See more from this season</button>
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
        height: 35.1vh;
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

    .wrapper .carousel .carousel-item h3 {
        font-size: 5vw;
        margin: 0;
        width: 55vw;
        text-wrap: wrap;
        color: var(--headings);
    }

    .wrapper .carousel .carousel-item p {
        font-size: 3vw;
        color: var(--sub-text);
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
    .indicator {
        position: relative;
        width: 0.5rem;
        height: 0.5rem;
        background: transparent;
        margin: 0 0.1rem;
        cursor: pointer;
        border: none;
    }
    
    .indicator span {
        position: absolute;
        display: block;
        background-color: white;
        width: 0.5rem;
        height: 0.1rem;
        transition: height 0.3s ease;
    }
    .indicator.active span {
        height: 0.5rem;
        background-color: var(--primary);
    }

    .indicators {
        position: absolute;
        bottom: 0.5rem;
        left: 50%;
        transform: translate(-50%, 0);
    }

    .more-this-season {
        width: 100vw;
        display: flex;
        justify-content: center;
        align-items: center;
    }


</style>
