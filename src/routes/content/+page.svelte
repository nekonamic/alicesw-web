<script lang="ts">
    import { page } from "$app/state";
    import { goto } from "$app/navigation";

    let { data } = $props();
    const kw = $derived(page.url.searchParams.get("kw"));
    // const page = $derived($page.url.searchParams.get("page"));

    async function handleRead(novel_id: Number, chapter_id: String) {
        await goto(`/${novel_id}/${chapter_id}`);
    }
</script>

<div class="container mx-auto p-6">
    <h1 class="text-3xl font-bold mb-8">"{kw}"</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each data.result as item (item.chapterId)}
            <div
                class="card bg-base-200 shadow-lg hover:shadow-xl transition-shadow duration-300"
            >
                <div class="card-body">
                    <div class="badge badge-primary mb-3">
                        {item.author}
                    </div>

                    <h2 class="card-title text-lg line-clamp-2">
                        {item.novelTitle}
                    </h2>

                    <p class="text-sm text-base-content/70 line-clamp-2">
                        {item.chapterTitle}
                    </p>

                    <div class="divider my-2"></div>

                    <div
                        class="flex justify-between items-center text-xs text-base-content/60"
                    >
                        <span>章节 #{item.chapterIndex}</span>
                        <span>ID: {item.chapterId}</span>
                    </div>

                    <div class="card-actions justify-end mt-4">
                        <button
                            class="btn btn-sm btn-primary"
                            onclick={() =>
                                handleRead(item.novelId, item.chapterId)}
                            >阅读</button
                        >
                    </div>
                </div>
            </div>
        {/each}
    </div>
</div>
