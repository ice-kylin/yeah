<script lang="ts">
    import Google from "@icons/Google.svelte";
    import Magnify from "@icons/Magnify.svelte";
</script>

<div
        class="bg-surfaceContainerHigh/60 backdrop-blur-lg rounded-lg max-w-3xl mx-auto my-32 px-3.5 py-2.5 flex items-center gap-3.5"
>
    <div class="text-primary">
        <Google/>
    </div>
    <label class="hidden" for="search">搜索</label>
    <textarea
            aria-autocomplete="both"
            autocapitalize="off"
            autocomplete="off"
            autofocus
            class="bg-transparent w-full outline-none resize-none"
            cols="1"
            id="search"
            name="search"
            placeholder="搜索，从这里开始..."
            rows="1"
            spellcheck="false"
    ></textarea>
    <div class="text-onSurfaceVariant">
        <Magnify/>
    </div>
</div>
