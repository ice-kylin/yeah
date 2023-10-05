<script lang="ts">
    import axios from "axios";
    import LinkCard from "./LinkCard.svelte";
    import type {Group} from "@src/types/config.ts";


    let promise = (async () => {
        try {
            return (await axios.get<Group[]>("http://192.168.3.4:3000/")).data;
        } catch (e) {
            throw new Error("Failed to fetch groups");
        }
    })();
</script>

{#await promise}
    <div></div>
{:then groups}
    {#each groups as group}
        <h2 class="px-6">{group.name}</h2>
        <div
                class="grid grid-cols-[repeat(auto-fit,_minmax(256px,_1fr))] items-center border-b border-b-outlineVariant border-dotted"
        >
            {#each group.items as item}
                <LinkCard name={item.name} url={item.url} description={item.description} logo={item.logo}></LinkCard>
            {/each}
        </div>
        <div class="absolute w-0.5 h-full bg-background right-0 top-0"></div>
        <div class="absolute w-0.5 h-full bg-background left-0 top-0"></div>
    {/each}
{:catch error}
    <div>{error}</div>
{/await}
