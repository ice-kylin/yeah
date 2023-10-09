<script lang="ts">
    import axios from "axios";
    import type {Group} from "@src/types/group.ts";
    import Link from "@components/service/Link.svelte";

    let groups = (async () => {
        try {
            return (await axios.get<Group[]>(`http://${location.hostname}:3000/`)).data;
        } catch (e) {
            console.error(e);
            throw new Error("Failed to get groups data: " + e);
        }
    })();
</script>

{#await groups}
    <div></div>
{:then groups}
    {#each groups as group}
        <h2 class="px-6 font-medium">{group.name}</h2>
        <div
                class="grid grid-cols-[repeat(auto-fit,_minmax(256px,_1fr))] items-center border-b border-b-outlineVariant border-dotted"
        >
            {#each group.items as item}
                {#if item.Link !== undefined}
                    <Link link={item.Link}/>
                {/if}
            {/each}
        </div>
        <div class="absolute w-0.5 h-full bg-background right-0 top-0"></div>
        <div class="absolute w-0.5 h-full bg-background left-0 top-0"></div>
    {/each}
{:catch error}
    <div>{error}</div>
{/await}
