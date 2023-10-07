<script lang="ts">
    import Google from "@icons/Google.svelte";
    import Magnify from "@icons/Magnify.svelte";
</script>

<div
        class="bg-surfaceContainerHigh/60 backdrop-blur-lg rounded-lg max-w-3xl mx-auto my-32 px-3.5 py-2.5 flex items-center gap-3.5 md:my-64"
>
    <div class="text-primary">
        <Google/>
    </div>
    <label for="search" class="hidden">搜索</label>
    <textarea
            name="search"
            id="search"
            cols="1"
            rows="1"
            class="bg-transparent w-full outline-none resize-none"
            aria-autocomplete="both"
            autocapitalize="off"
            autocomplete="off"
            spellcheck="false"
            autofocus
            placeholder="搜索，从这里开始..."
    ></textarea>
    <div class="text-onSurfaceVariant">
        <Magnify/>
    </div>
</div>
