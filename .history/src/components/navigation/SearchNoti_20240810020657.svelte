<script lang="ts">
    import FilledBell from '$lib/assets/icons/bell-filled.svg?component';
    import LinedBell from '$lib/assets/icons/bell.svg?component';
    import FilledNotiBell from '$lib/assets/icons/bell-notification-social-media-solid.svg?component';
    import LinedNotiBell from '$lib/assets/icons/bell-notification-social-media.svg?component';
    import SearchSVG from '$lib/assets/icons/search.svg?component';

    import { DisplayStateCSS, PageIndex, PageStore } from '$lib';
    import { get } from 'svelte/store';
    import { onMount } from 'svelte';

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
</script>

<section class="SEARCH_NOTI_BAR">
    <button class="bell" on:click={openNotifications} on:pointerenter={() => NotificationIcon.state = DisplayStateCSS.SHOW} on:pointerleave={() => NotificationIcon.state = DisplayStateCSS.HIDE}>
        {#if !NotificationIcon.hasNotification}
            <svelte:component this={NotificationIcon.no.selected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "show" : "hide"}/>
            <svelte:component this={NotificationIcon.no.unSelected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "hide" : "show"}/>
        {:else}
            <svelte:component this={NotificationIcon.yes.selected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "show" : "hide"}/>
            <svelte:component this={NotificationIcon.yes.unSelected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "hide" : "show"}/>
        {/if}
    </button>

</section>

<style>
    section {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        display: flex;
        justify-content: flex-end;
        padding: 1em;
        padding-top: 2rem;
        box-sizing: border-box;
    }

    section .bell {
        position: relative;
        width: 3em;
        height: 3em;
        background: transparent;
        border: none;

    } 

    :global(.SEARCH_NOTI_BAR svg) {
        width: 3em;
        height: 3em;
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }
    :global(.SEARCH_NOTI_BAR svg.show) {
        fill: var(--primary);
        opacity: 1;
    }   
    :global(.SEARCH_NOTI_BAR svg.hide) {
        fill: var(--icon-unselected);
        opacity: 0;
    }    
</style>