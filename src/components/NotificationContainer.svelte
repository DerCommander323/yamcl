<div class="absolute z-30 left-2 bottom-2">
    {#each $notificationStore as notif}
        <Notification {notif} />
    {/each}
</div>

<script>
    import Notification from "./Notification.svelte";
    import { notificationStore } from "../scripts/notificationSystem";
</script>