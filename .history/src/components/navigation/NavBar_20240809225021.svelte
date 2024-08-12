<script lang="ts">
    import FilledHouse from '$lib/assets/icons/house-blank-filled.svg?component';
    import LinedHouse from '$lib/assets/icons/house-blank.svg?component';
    import FilledCalendar from '$lib/assets/icons/calendar-solid.svg?component';
    import LinedCalendar from '$lib/assets/icons/calendar-thin.svg?component';
    import FilledDownload from '$lib/assets/icons/inbox-in-filled.svg?component';
    import LinedDownload from '$lib/assets/icons/inbox-in.svg?component';
    import FilledProfile from '$lib/assets/icons/circle-user-filled.svg?component';
    import LineProfile from '$lib/assets/icons/circle-user.svg?component';
    import { onMount, type ComponentType } from 'svelte';
    import { PageIndex, PageStore } from '$lib';
    import { get } from 'svelte/store';

    let path: PageIndex = get(PageStore);
    PageStore.subscribe((value) => {
        console.log(value);
        path = value;
    })

    const navOptions: { name: string, href: PageIndex, selected: ComponentType, unSelected: ComponentType }[] = [
        {
            name: 'Home',
            href: PageIndex.HOME,
            selected: FilledHouse,
            unSelected: LinedHouse
        },
        {
            name: 'Releases',
            href: PageIndex.RELEASES,
            selected: FilledCalendar,
            unSelected: LinedCalendar
            
        },
        {
            name: 'Downloads',
            href: PageIndex.DOWNLOADS,
            selected: FilledDownload,
            unSelected: LinedDownload
        },
        {
            name: 'Profile',
            href: PageIndex.PROFILE,
            selected: FilledProfile,
            unSelected: LineProfile
        }
    ]




    function navigateTo(page: PageIndex) {
        PageStore.set(page);
    }
</script>

<nav class="nav_menu">
    {#each navOptions as option}
        <button  class={path === option.href ? "selected" : "unselected"} on:click={()=>navigateTo(option.href)}>
            <svelte:component this={path === option.href ? option.selected : option.unSelected} name={option.name}/> 
            <span>{option.name}</span>
        </button>
    {/each}
</nav>

<style>
    nav {
        display: flex;
        justify-content: space-between;
        box-sizing: border-box;
        padding: 0 10vw;
        position: fixed;
        bottom: 0;
        left: 0;
        margin: 0;
        width: 100vw;
        background-color: rgba(18, 18, 18, 0.8);
        backdrop-filter: blur(16px);
    }

    nav button span {
        color: white;
        font-size: 0.6rem;
        text-decoration: none;
    }

    nav button.selected span {
        color: var(--primary);
    }
    nav button.unselected span {
        color: var(--icon-unselected);
    }
    nav button:hover span {
        color: var(--primary);
    }

    nav button {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-decoration: none;
        padding: 0.5rem;
        background-color: transparent;
        border: none;
    }

    button.unselected:hover span, button.unselected:active span {
        color: var(--primary);
        fill: var(--primary);
    }

    :global(.nav_menu button.selected svg) {
        fill: var(--primary);
        width: 1.5rem;
        height: auto;
    }

    :global(.nav_menu button.unselected svg) {
        fill: var(--icon-unselected);
        width: 1.75rem;
        height: auto;
    }

    :global(.nav_menu button.unselected:hover svg , .nav_menu button.unselected:active svg) {
        fill: var(--primary);
    }

</style>