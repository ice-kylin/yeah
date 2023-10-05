<script lang="ts">
    export let name: string;
    export let url: string;
    export let description: string | undefined;
</script>

<a href={url} target="_blank" rel="noopener noreferrer">
    <div
            class="py-4 border-t border-dotted border-t-outlineVariant hover:bg-surface transition-colors"
    >
        <div
                class="px-6 border-r border-dotted border-r-outlineVariant flex flex-col justify-center gap-2"
        >
            <div class="bg-primaryContainer w-6 h-6 rounded">
                <p class="text-center font-emj">🥝</p>
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
