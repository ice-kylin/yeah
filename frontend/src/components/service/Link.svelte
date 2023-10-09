<script lang="ts">
    import type {Link} from "@src/types/service/link.ts";
    import {getHref} from "@utils/href.ts";
    import {getRandomEmoji} from "@utils/emoji.ts";

    export let link: Link;

    $: href = getHref(link.href);
    $: hide = link.hide && href === undefined;
</script>

<a {href} rel="noopener noreferrer" target="_blank" class:hidden={hide}>
    <div
            class="py-4 border-t border-dotted border-t-outlineVariant hover:bg-patten-surface bg-[length:2px_2px] transition-colors"
    >
        <div
                class="px-6 border-r border-dotted border-r-outlineVariant flex flex-col justify-center gap-2"
        >
            <div class="bg-surfaceContainer w-6 h-6 rounded">
                {#if link.logo === null}
                    <p class="text-center font-emj">{getRandomEmoji()}</p>
                {:else if link.logo?.Emj === undefined}
                    <p class="text-center font-emj">🥝</p>
                {:else}
                    <p class="text-center font-emj">{link.logo.Emj}</p>
                {/if}
            </div>
            <h3 class="font-medium">{link.name}</h3>
            <p class="text-onSurfaceVariant text-xs overflow-hidden text-ellipsis whitespace-nowrap">
                {#if link.description !== null}
                    {link.description}
                {:else}
                    &nbsp;
                {/if}
            </p>
            <p class="text-outline text-xs font-mono overflow-hidden text-ellipsis whitespace-nowrap">
                {#if href !== undefined}
                    {href}
                {:else}
                    Unavailable here
                {/if}
            </p>
        </div>
    </div>
</a>
