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
    import { PageIndex, History } from '$lib';

    let path: PageIndex;
    
    const navOptions: { name: string, href: PageIndex, state: boolean, selected: ComponentType, unSelected: ComponentType }[] = [
        {
            name: 'Home',
            href: PageIndex.HOME,
            state: true,
            selected: FilledHouse,
            unSelected: LinedHouse
        },
        {
            name: 'Releases',
            href: PageIndex.RELEASES,
            state: true,
            selected: FilledCalendar,
            unSelected: LinedCalendar
            
        },
        {
            name: 'Downloads',
            href: PageIndex.DOWNLOADS,
            state: true,
            selected: FilledDownload,
            unSelected: LinedDownload
        },
        {
            name: 'Profile',
            href: PageIndex.PROFILE,
            state: true,
            selected: FilledProfile,
            unSelected: LineProfile
        }
    ]

    onMount(async () => {
        path = await History.get();
    })

    History.onUpdate(value => {
        path = value;
        for (const option in navOptions) {
            if (navOptions[option].href === path) {
                navOptions[option].state = true;
            } else {
                navOptions[option].state = false;
            }
        }
    })

    function navigateTo(page: PageIndex) {
        History.push(page);
    }

    
</script>

<nav class="nav_menu">
    {#each navOptions as option}
        <button  class={path === option.href ? "selected" : "unselected"} on:click={()=>navigateTo(option.href)}>
            <div>
                <svelte:component this={option.selected} name={option.name} class={option.state ? "show" : "hide"}/> 
                <svelte:component this={option.unSelected} name={option.name} class={option.state ? "hide" : "show"}/> 
            </div>
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
        padding-bottom: 2vh;
        padding-top: 1vh;
        position: fixed;
        bottom: 0;
        left: 0;
        margin: 0;
        width: 100vw;
        background-color: rgba(18, 18, 18, 0.8);
        backdrop-filter: blur(5px);

        transition: all 0.3s ease;
        z-index: 10;
    }

    nav button span {
        color: white;
        font-size: 0.75rem;
        text-decoration: none;
        font-weight: 700;
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
        background-color: transparent;
        border: none;
        width: 3.25rem;
        height: 3.25rem;
        padding: 0;
    }

    button.unselected:hover span, button.unselected:active span {
        color: var(--primary);
        fill: var(--primary);
    }

    :global(.nav_menu button svg) {
        width: 1.85rem;
        height: auto;
        transition: opacity 0.25s ease;
    }

    :global(.nav_menu button.selected svg) {
        fill: var(--primary);
        
    }

    :global(.nav_menu button.unselected svg) {
        fill: var(--icon-unselected);
    }

    :global(.nav_menu button.unselected:hover svg , .nav_menu button.unselected:active svg) {
        fill: var(--primary);
    }

    .nav_menu button div {
        position: relative;
        width: 1.85rem;
        height: 1.85rem;
    }

    :global(.nav_menu button div svg) {
        position: absolute;
        top: 0;
        left: 0;
    }
    :global(.nav_menu button div svg.hide) {
        /* display: none; */
        opacity: 0;
    }
    :global(.nav_menu button div svg.show) {
        /* display: block; */
        opacity: 1;
    }

</style>