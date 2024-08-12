<script lang="ts">
    import FilledBell from '$lib/assets/icons/bell-filled.svg?component';
    import LinedBell from '$lib/assets/icons/bell.svg?component';
    import FilledNotiBell from '$lib/assets/icons/bell-notification-social-media-solid.svg?component';
    import LinedNotiBell from '$lib/assets/icons/bell-notification-social-media.svg?component';

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
        hasNotification: false,
        state: DisplayStateCSS.HIDE
    }
</script>

<section class="SEARCH_NOTI_BAR">
    <button class="bell">
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
    }

    section .bell {
        width: 3.5em;
        height: 3.5em;
    } 

    :global(.SEARCH_NOTI_BAR svg) {
        width: 3.5em;
        height: 3.5em;
        fill: var(--icon-unselected);
        position: absolute;
    }
    :global(.SEARCH_NOTI_BAR svg.show) {
        opacity: 1;
    }   
    :global(.SEARCH_NOTI_BAR svg.hide) {
        opacity: 0;
    }    
</style>