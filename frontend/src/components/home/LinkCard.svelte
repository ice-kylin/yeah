<script lang="ts">
    import type {Logo} from "../../types/config.ts";
    import {getRandomEmoji} from "@utils/emoji.ts";

    export let name: string;
    export let url: string;
    export let description: string | null;
    export let logo: Logo | null;
</script>

<a href={url} rel="noopener noreferrer" target="_blank">
    <div
            class="py-4 border-t border-dotted border-t-outlineVariant hover:bg-surface transition-colors"
    >
        <div
                class="px-6 border-r border-dotted border-r-outlineVariant flex flex-col justify-center gap-2"
        >
            <div class="bg-surfaceContainer w-6 h-6 rounded">
                {#if logo === null}
                    <p class="text-center font-emj">{getRandomEmoji()}</p>
                {:else if logo?.Emj === undefined}
                    <p class="text-center font-emj">🥝</p>
                {:else}
                    <p class="text-center font-emj">{logo.Emj}</p>
                {/if}
            </div>
            <h3>{name}</h3>
            <p class="text-outlineVariant text-xs font-mono">
                {description}
            </p>
            <p class="text-outlineVariant text-xs font-mono">
                {url}
            </p>
        </div>
    </div>
</a>
