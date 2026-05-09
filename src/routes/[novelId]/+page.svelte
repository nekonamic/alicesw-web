<script lang="ts">
  import { favorites } from "$lib/utils/favorites.svelte";
  import { page } from "$app/state";
  let { data } = $props();

  const formatWordCount = (count: number) => {
    return count > 10000 ? (count / 10000).toFixed(1) + "万" : count;
  };
</script>

<div class="max-w-4xl mx-auto p-4 md:p-8 space-y-8">
  <header>
    <div class="card-body">
      <div class="flex justify-between items-start">
        <div>
          <h1 class="card-title text-3xl font-bold">
            {data.novel_title}
          </h1>
          <a
            class="text-primary text-lg mt-1 underline"
            href="/author?kw={data.author}&page=1"
          >
            {data.author}
          </a>
        </div>
      </div>

      <div class="stats shadow">
        <div class="stat">
          <div class="stat-figure text-primary">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="size-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z"
              />
            </svg>
          </div>
          <div class="stat-title">总字数</div>
          <div class="stat-value text-primary">
            {formatWordCount(data.word_count)}
          </div>
        </div>

        <div class="stat">
          <div class="stat-figure text-secondary">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="size-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z"
              />
            </svg>
          </div>
          <div class="stat-title">章节数</div>
          <div class="stat-value text-secondary">{data.chapter_count}</div>
        </div>
      </div>

      <div class="flex justify-end mt-6 space-x-2">
        <details class="dropdown dropdown-center">
          <summary class="btn">下载</summary>
          <ul
            class="menu dropdown-content bg-base-200 rounded-box z-1 p-2 shadow-sm"
          >
            <li><a href="/api/epub/{data.novel_id}">EPUB</a></li>
            <li><a href="/api/txt/novel/{data.novel_id}">TXT</a></li>
          </ul>
        </details>
        {#if favorites.includes(Number(page.params.novelId))}
          <button
            class="btn btn-soft"
            onclick={() => {
              favorites.remove(Number(page.params.novelId));
            }}
          >
            取消收藏
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="size-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="m3 3 1.664 1.664M21 21l-1.5-1.5m-5.485-1.242L12 17.25 4.5 21V8.742m.164-4.078a2.15 2.15 0 0 1 1.743-1.342 48.507 48.507 0 0 1 11.186 0c1.1.128 1.907 1.077 1.907 2.185V19.5M4.664 4.664 19.5 19.5"
              />
            </svg>
          </button>
        {:else}
          <button
            class="btn"
            onclick={() => {
              favorites.addById(data.novel_title, Number(page.params.novelId));
            }}
          >
            添加收藏
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="size-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0 1 11.186 0Z"
              />
            </svg>
          </button>
        {/if}
        <a
          class="btn btn-primary"
          href="/{data.novel_id}/{data.chapter_info.find(
            (ch) => ch.chapter_index === 0,
          )?.id}">开始阅读</a
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
          <span class="text-xs opacity-50 text-nowrap"
            >{formatWordCount(chapter.word_count)} 字</span
          >
        </a>
      {/each}
    </div>
  </section>
</div>
