<div class="fixed z-30 left-0 bottom-0 m-2 flex flex-col">
    {#each $notificationStore as notif}
        <Notification {notif} />
    {/each}
</div>

<script>
    import Notification from "./Notification.svelte";
    import { notificationStore } from "../scripts/notificationSystem";
</script>