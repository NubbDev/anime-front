<script lang="ts">
    import { clipTitle, PageIndex, History, type AnimeCardInfo, BodyScroll } from "$lib";

    import { get, writable } from "svelte/store";
    import BannerImage from "./BannerImage.svelte";
    export let anime: AnimeCardInfo[];

    let currentSlide = writable(0);
    let previousSlide = 0;
    let slideChange = 0;
    let currentindex = 0; 
    let offset = 0;

    let isDragStart = false, isDragging = false, prevPageX: number, prevScollLeft: number, positionDiff: number;

    let carousel: HTMLDivElement;

    let ImgWidth: number;

    let showMoreText = false;

    const dragStart = (e: TouchEvent | MouseEvent) => {
        e.preventDefault();
        isDragStart = true;
        if (e instanceof MouseEvent) {
            prevPageX = e.pageX;
        } else if (e instanceof TouchEvent) {
            prevPageX = e.touches[0].pageX;
        }
        prevScollLeft = carousel.scrollLeft;
    }

    const dragging = (e: TouchEvent | MouseEvent) => {
        e.preventDefault();
        if (!isDragStart) return;
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
            showMoreText = true;
            return currentSlide.set(0);
        }
        
        if (carousel) {
            currentindex = value;
            return carousel.scrollLeft = ImgWidth * value;
        } 
    })

    BodyScroll.subscribe(value => {
        offset = value;
    })

    
</script>

<svelte:document on:mouseup={dragStop} on:touchend={dragStop} on:mousemove={dragging}/>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<section class="wrapper">
    <div class="carousel" bind:this={carousel} style="scroll-behavior: smooth;" on:mousedown={dragStart} on:touchstart={dragStart} on:touchmove={dragging}>
        {#each anime as anime, i}
            <div class="carousel-item {i === get(currentSlide) ? 'active' : ''}" bind:clientWidth={ImgWidth} >
                <BannerImage banner={anime.bannerImage} title={anime.title.english ?? anime.title.romaji} genres={anime.genres}/>
            </div>
        {/each}
    </div>
    {#if anime.length > 1}
        <div class="indicators">
            {#each anime as _, i}
                <button class="indicator {i == currentindex ? 'active' : ''}" on:click={() => currentSlide.set(i)}><span/></button>
            {/each}
        </div>
    {/if}
</section>
<section class="more-this-season {showMoreText ? 'show' : 'hide'}">
    <button on:click={() => History.push(PageIndex.SEASON)}>See more from this season</button>
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
        /* transition: translateX 0.5s; */
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

    .indicator {
        position: relative;
        width: clamp(1rem, 2vw, 3rem);
        height: 0.75rem;
        background: transparent;
        cursor: pointer;
        border: none;
        padding: 0;
        margin: 0 0.2rem;
        
    }
    
    .indicator span {
        position: absolute;
        bottom: 0rem;
        left: 0;
        display: block;
        background-color: white;
        width: 100%;
        height: 0.1rem;
        
        transition: height 0.3s ease;
    }
    .indicator.active span {
        height: 100%;
        background-color: var(--primary);
    }

    .indicators {
        position: absolute;
        bottom: 0.5rem;
        left: 50%;
        transform: translate(-50%, 0);
        z-index: 5;
        height: 0.5rem;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .more-this-season {
        width: 100vw;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: var(--primary);
        transition: opacity 1s ease, position 1s ease;
        
    }
    .more-this-season button {
        background-color: var(--primary);
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        font-size: 3vw;
        cursor: pointer;
        /* transition: all 0.3s ease; */
    }

    .more-this-season.show {
        position: relative;
        opacity: 1;
        cursor: pointer;
    }
    .more-this-season.hide {
        position: absolute;
        opacity: 0;
        cursor: none;
    }


</style>
