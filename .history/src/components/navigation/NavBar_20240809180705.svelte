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
  import { page } from '$app/stores';

    let path: string = '';

    const navOptions: { name: string, href: string, selected: ComponentType, unSelected: ComponentType }[] = [
        {
            name: 'Home',
            href: '/',
            selected: FilledHouse,
            unSelected: LinedHouse
        },
        {
            name: 'Releases',
            href: '/releases',
            selected: FilledCalendar,
            unSelected: LinedCalendar
            
        },
        {
            name: 'Downloads',
            href: '/downloads',
            selected: FilledDownload,
            unSelected: LinedDownload
        },
        {
            name: 'Profile',
            href: '/profile',
            selected: FilledProfile,
            unSelected: LineProfile
        }
    ]


    // onMount(() => {
    //     path = '/' + window.location.pathname.split('/')[1];
    //     console.log(navOptions[0].selected);
    // })

    $: path = $page.url.pathname ?? '/';
    $: console.log(path);
</script>

<nav class="nav_menu">
    {#each navOptions as option}
        <a href={option.href} class={path === option.href ? "selected" : "unselected"}>
            <svelte:component this={path === option.href ? option.selected : option.unSelected} name={option.name}/> 
            <span>{option.name}</span>
        </a>
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
        backdrop-filter: 16px;
    }

    nav a span {
        color: white;
        font-size: 0.6rem;
        text-decoration: none;
    }
    nav a span:hover {
        color: var(--icon-selected);
    }

    nav a {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-decoration: none;
        padding: 0.5rem;
    }

    :global(.nav_menu a.selected svg) {
        fill: var(--icon-selected);
        width: 1.5rem;
        height: auto;
    }

    :global(.nav_menu a.unselected svg) {
        fill: white;
        width: 1.5rem;
        height: auto;
    }
</style>