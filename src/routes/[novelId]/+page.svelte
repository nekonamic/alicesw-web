<script lang="ts">
    let { data } = $props();
    import { goto } from "$app/navigation";

    const formatWordCount = (count: number) => {
        return count > 10000 ? (count / 10000).toFixed(1) + "万" : count;
    };

    async function handleStart() {
        await goto(
            `/${data.novel_id}/${data.chapter_info.find((ch) => ch.chapter_index === 0)?.id}`,
        );
    }

    async function handleChapter(id: number) {
        await goto(`/${data.novel_id}/${id}`);
    }
</script>

<div class="max-w-4xl mx-auto p-4 md:p-8 space-y-8">
    <header>
        <div class="card-body">
            <div class="flex justify-between items-start">
                <div>
                    <h1 class="card-title text-3xl font-bold">
                        {data.novel_title}
                    </h1>
                    <p class="text-primary font-medium mt-1">
                        作者：{data.author}
                    </p>
                </div>
            </div>

            <div class="stats shadow bg-base-200/50 mt-4">
                <div class="stat place-items-center p-4">
                    <div class="stat-title text-xs">总字数</div>
                    <div class="stat-value text-xl text-info">
                        {formatWordCount(data.word_count)}
                    </div>
                </div>
                <div class="stat place-items-center p-4">
                    <div class="stat-title text-xs">章节数</div>
                    <div class="stat-value text-xl text-info">
                        {data.chapter_count}
                    </div>
                </div>
            </div>

            <div class="card-actions justify-end mt-6">
                <button class="btn btn-primary px-8" onclick={handleStart}
                    >开始阅读</button
                >
            </div>
        </div>
    </header>

    <section class="space-y-4">
        <div
            class="flex items-center justify-between border-b border-base-300 pb-2"
        >
            <h2 class="text-xl font-bold flex items-center gap-2">
                正文目录
                <span class="text-sm font-normal opacity-60"
                    >共 {data.chapter_info.length} 章</span
                >
            </h2>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
            {#each data.chapter_info as chapter}
                <a
                    href="/{data.novel_id}/{chapter.id}"
                    class="group flex items-center justify-between p-4 bg-base-100 border border-base-200 rounded-lg hover:border-primary hover:bg-primary/5 transition-all"
                >
                    <div class="flex items-center gap-3">
                        <span class="text-sm opacity-40 font-mono"
                            >#{chapter.chapter_index + 1}</span
                        >
                        <span class="group-hover:text-primary transition-colors"
                            >{chapter.title}</span
                        >
                    </div>
                    <span class="text-xs opacity-50"
                        >{formatWordCount(chapter.word_count)} 字</span
                    >
                </a>
            {/each}
        </div>
    </section>
</div>
