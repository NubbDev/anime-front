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
        state: DisplayStateCSS.SHOW
    }
</script>

<section class="SEARCH_NOTI_BAR">
    <div class="bell">
        {#if NotificationIcon.hasNotification}
            <svelte:component this={NotificationIcon.no.selected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "show" : "hide"}/>
            <svelte:component this={NotificationIcon.no.unSelected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "hide" : "show"}/>
        {:else}
            <svelte:component this={NotificationIcon.yes.selected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "show" : "hide"}/>
            <svelte:component this={NotificationIcon.yes.unSelected} class={NotificationIcon.state == DisplayStateCSS.SHOW ? "hide" : "show"}/>
        {/if}
    </div>

</section>

<style>
    section {
        position: fixed;
        top: 0;
        right: 0;
    }
</style>