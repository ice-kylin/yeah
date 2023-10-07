<script lang="ts">
    import {onDestroy} from "svelte";

    let currentTime = new Date();

    const dateFormatter = new Intl.DateTimeFormat(undefined, {
        weekday: "short",
        year: "numeric",
        month: "long",
        day: "numeric",
        hour: "numeric",
        minute: "numeric",
        second: "numeric",
    });

    function updateTime() {
        currentTime = new Date();
    }

    const interval = setInterval(updateTime, 1000);

    onDestroy(() => {
        clearInterval(interval);
    });
</script>

<p class="text-sm font-mono">
    {dateFormatter.format(currentTime)}
</p>
