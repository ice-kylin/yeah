<script lang="ts">
    import axios from "axios";
    import type {Message} from "@src/types/message.ts";

    let messages = (async () => {
        try {
            return (await axios.get<Message[]>(`http://${location.hostname}:3000/message`)).data;
        } catch (e) {
            console.error(e);
            throw new Error("Failed to get groups data: " + e);
        }
    })();
</script>

<div>
    {#await messages then messages}
        {#each messages as message}
            <div class="bg-patten-primary bg-[length:2px_2px] px-6 py-4">
                <h2 class="font-medium">{message.title}</h2>
                <p class="text-sm mt-2">{message.content}</p>
            </div>
        {/each}
    {/await}
</div>
