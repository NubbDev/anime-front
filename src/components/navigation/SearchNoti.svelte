<script lang="ts">
    import FilledBell from '$lib/assets/icons/bell-filled.svg?component';
    import LinedBell from '$lib/assets/icons/bell.svg?component';
    import FilledNotiBell from '$lib/assets/icons/bell-notification-social-media-solid.svg?component';
    import LinedNotiBell from '$lib/assets/icons/bell-notification-social-media.svg?component';
    import SearchSVG from '$lib/assets/icons/search.svg?component';

    import { BodyScroll, PageIndex, History, AppSearchStore} from '$lib';
    import { onMount } from 'svelte';

    // export let socket: WebSocket;
    

    let showBackDrop: "hide" | "show" = "hide";
    let pastScroll = 0;
    let shouldBackgroundMove = false;
    let display: 'hide' | 'show' = 'show';
    let show = true;

    const NotificationIcon = {
        yes: {
            selected: FilledNotiBell,
            unSelected: LinedNotiBell
        },
        no: {
            selected: FilledBell,
            unSelected: LinedBell
        },
        hasNotification: true,
        state: false
    }

    const SearchIcon = {
        selected: SearchSVG,
        state: false
    }


    const setNotification = (hasNotification: boolean) => {
        NotificationIcon.hasNotification = hasNotification;
    }

    const openNotifications = () => {
        console.log("Open Notifications");
    }

    const displaySearchBar = () => {
        AppSearchStore.set(true);
    }
    onMount(() => {
        let threshold = window.innerHeight * 0.1;
        BodyScroll.subscribe(value => {
            if (value > threshold && !shouldBackgroundMove) {
                shouldBackgroundMove = true;
                showBackDrop = "show";
                
            } else if (value < threshold && shouldBackgroundMove) {
                shouldBackgroundMove = false;
                showBackDrop = "hide";
            }
            
            threshold = window.innerHeight * 0.1;
        })

        display = 'hide';
        show = false;
        
    })

    History.onUpdate(value => {
        // if (value == PageIndex.ANIME || value == PageIndex.PLAYER) {
        //     display = 'hide';
        //     setTimeout(() => {
        //         show = false;
        //     }, 650)
        // } else {
        //     show = true;
        //     setTimeout(() => {
        //         display = 'show';
        //     }, 10)
        // }
    })    
</script>

{#if show}
<!-- svelte-ignore a11y-no-static-element-interactions -->
<section class="SEARCH_NOTI_BAR {display}">
    <span class="SEARCH_NOTI_BAR_Background {showBackDrop}"/>
    <button class="bell" on:click={openNotifications} on:pointerenter={() => NotificationIcon.state = true} on:pointerleave={() => NotificationIcon.state = false}>
        {#if !NotificationIcon.hasNotification}
            <svelte:component this={NotificationIcon.no.selected} class={NotificationIcon.state ? "show" : "hide"}/>
            <svelte:component this={NotificationIcon.no.unSelected} class={NotificationIcon.state ? "hide" : "show"}/>
        {:else}
            <svelte:component this={NotificationIcon.yes.selected} class={NotificationIcon.state ? "show" : "hide"}/>
            <svelte:component this={NotificationIcon.yes.unSelected} class={NotificationIcon.state ? "hide" : "show"}/>
        {/if}
    </button>
    
    <button class="searchbutton" on:click={displaySearchBar}>
        <svelte:component this={SearchIcon.selected} />
    </button>
</section>
{/if}

<style>
    section.SEARCH_NOTI_BAR {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        display: flex;
        justify-content: flex-end;
        padding-right: 10vw;
        padding-top: 7vh;
        box-sizing: border-box;
        gap: 2em;
        z-index: 10;
        transition: transform 0.5s ease, opacity 0.5s ease;
    }

    section.SEARCH_NOTI_BAR.hide {
        opacity: 0;
    }

    section.SEARCH_NOTI_BAR.show {
        opacity: 1;
    }

    section.SEARCH_NOTI_BAR .SEARCH_NOTI_BAR_Background {
        position: absolute;
        top: 0;
        left: 50%;
        /* border-radius: 10rem; */
        background: rgba(0, 0, 0, 0.5);
        z-index: -1;
        backdrop-filter: blur(10px);
        transition: height 0.5s ease, transform 0.5s ease;
        width: 100%;
        height: 120%;
    }

    section.SEARCH_NOTI_BAR .SEARCH_NOTI_BAR_Background.hide {
        height: 0;
        transform: translate(-50%, -50%);
    }
    section.SEARCH_NOTI_BAR .SEARCH_NOTI_BAR_Background.show {
        width: 100%;
        height: 120%;
        transform: translate(-50%, 0%);
    }

    section.SEARCH_NOTI_BAR button {
        position: relative;
        width: 2em;
        height: 2em;
        background: transparent;
        border: none;

    }

    :global(section.SEARCH_NOTI_BAR svg) {
        width: 2em;
        height: 2em;
        fill: var(--icon-unselected);
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }
    :global(section.SEARCH_NOTI_BAR svg.show) {
        opacity: 1;
    }   
    :global(section.SEARCH_NOTI_BAR svg.hide) {
        opacity: 0;
    }

    
</style>