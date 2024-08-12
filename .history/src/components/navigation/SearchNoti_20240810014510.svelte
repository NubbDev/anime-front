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

        }
        no: {
            
        }
        selected: {
            Notification: FilledNotiBell,
            Bell: FilledBell
        },
        unSelected: {
            Notification: LinedNotiBell,
            Bell: LinedBell
        },
        hasNotification: false,
        state: DisplayStateCSS.SHOW
    }
</script>

<section>
    <div class="bell">
        {#if NotificationIcon.hasNotification}
            <svelte:component/>

        {:else}
            
        {/if}
    </div>

</section>