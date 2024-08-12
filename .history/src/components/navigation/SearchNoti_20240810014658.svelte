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

<section>
    <div class="bell">
        {#if NotificationIcon.hasNotification}
            <svelte:component this={NotificationIcon.no.selected}/>
            <svelte:component this={NotificationIcon.no.unSelected}/>
        {:else}
            <svelte:component this={NotificationIcon.yes.selected}/>
            <svelte:component this={NotificationIcon.yes.unSelected}/>
        {/if}
    </div>

</section>