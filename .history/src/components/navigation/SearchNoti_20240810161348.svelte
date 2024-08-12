<script lang="ts">
    import FilledBell from '$lib/assets/icons/bell-filled.svg?component';
    import LinedBell from '$lib/assets/icons/bell.svg?component';
    import FilledNotiBell from '$lib/assets/icons/bell-notification-social-media-solid.svg?component';
    import LinedNotiBell from '$lib/assets/icons/bell-notification-social-media.svg?component';
    import SearchSVG from '$lib/assets/icons/search.svg?component';

    import { DisplayStateCSS, PageIndex, PageStore } from '$lib';
    import { get } from 'svelte/store';
    import { onMount } from 'svelte';

    let searchBarShow: DisplayStateCSS = DisplayStateCSS.HIDE;

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
        state: DisplayStateCSS.HIDE
    }

    const SearchIcon = {
        selected: SearchSVG,
        state: DisplayStateCSS.HIDE
    }

    const setNotification = (hasNotification: boolean) => {
        NotificationIcon.hasNotification = hasNotification;
    }

    const openNotifications = () => {
        console.log("Open Notifications");
    }

    const displaySearchBar = () => {
        searchBarShow = searchBarShow === DisplayStateCSS.SHOW ? DisplayStateCSS.HIDE : DisplayStateCSS.SHOW;
    }

    
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<section class="SEARCH_NOTI_BAR">
    <button class="bell {searchBarShow == DisplayStateCSS.SHOW ? 'hide' : 'show'}" on:click={openNotifications} on:pointerenter={() => NotificationIcon.state = DisplayStateCSS.SHOW} on:pointerleave={() => NotificationIcon.state = DisplayStateCSS.HIDE}>
        {#if !NotificationIcon.hasNotification}
            <svelte:component this={NotificationIcon.no.selected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "show" : "hide"}/>
            <svelte:component this={NotificationIcon.no.unSelected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "hide" : "show"}/>
        {:else}
            <svelte:component this={NotificationIcon.yes.selected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "show" : "hide"}/>
            <svelte:component this={NotificationIcon.yes.unSelected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "hide" : "show"}/>
        {/if}
    </button>
    
    <button class="searchbutton" on:click={displaySearchBar}>
        <svelte:component this={SearchIcon.selected} />
    </button>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="search_bar {searchBarShow == DisplayStateCSS.SHOW ? 'show' : 'hide'}" on:click={displaySearchBar}>
        <input type="text" placeholder="Search..."/>
        <div class="backdrop {searchBarShow == DisplayStateCSS.SHOW ? 'show' : 'hide'}"></div>
    </div>

</section>

<style>
    section {
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
    }

    section button {
        position: relative;
        width: 2em;
        height: 2em;
        background: transparent;
        border: none;

    }
    section .search_bar {
        position: absolute;
        top: 0;
        height: 2.5rem;
        transition: width 0.5s ease;
        
    }
    section .search_bar.hide {
        width: 2rem;
    }
    section .search_bar.show {
        width: 90vw;
        right: 5vw;
    }

    section .search_bar.show input {
        transition: all 0.5s ease;
        background: rgba(255, 255, 255, 0.5);
        border-color: #fff;
        opacity: 1;
    }

    section .search_bar.hide input {
        opacity: 0;
    }

    section .search_bar.show input:focus {
        border: #fff;
    }

    section .search_bar .backdrop {
        position: absolute;
        top: -7vh;
        left: -5vw;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.5);
        z-index: -1;
        backdrop-filter: blur(10px);
    }

    

    section .search_bar .backdrop.hide {
        width: 0;
        height: 0;
    }

    section .search_bar input {
        width: 100%;
        height: 100%;
        border: 1px solid var(--icon-unselected);
        border-radius: 2rem;
        padding: 0 1em;
        box-sizing: border-box;
    }

    :global(.SEARCH_NOTI_BAR svg) {
        width: 2em;
        height: 2em;
        fill: var(--icon-unselected);
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }
    :global(.SEARCH_NOTI_BAR svg.show) {
        opacity: 1;
    }   
    :global(.SEARCH_NOTI_BAR svg.hide) {
        opacity: 0;
    }    
</style>