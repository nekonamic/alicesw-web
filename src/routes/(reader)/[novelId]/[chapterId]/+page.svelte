<script lang="ts">
  import { page } from "$app/state";
  let { data } = $props();
</script>

<div class="py-6 px-2 sm:py-12">
  <article class="mx-auto max-w-3xl p-6 sm:p-12">
    <header class="mb-10 text-center">
      <h1 class="font-bold text-3xl text-base-content mb-4 leading-tight">
        {data.title}
      </h1>
      <div class="badge">
        章节: {data.chapter_index + 1} / {data.total_chapters}
      </div>
    </header>

    <section
      class="text-slate-800 dark:text-slate-300 leading-loose tracking-wide whitespace-pre-wrap mb-12"
    >
      {data.content}
    </section>

    <div class="divider"></div>

    <nav class="flex justify-between items-center mt-6">
      {#if data.prev_id}
        <a
          href={`/${page.params.novelId}/${data.prev_id}`}
          class="btn btn-outline sm:w-32"
        >
          « 上一章
        </a>
      {:else}
        <button
          class="btn btn-outline btn-disabled sm:w-32"
          aria-disabled="true"
        >
          已是第一章
        </button>
      {/if}

      <a
        href="/{page.params.novelId}"
        class="btn btn-ghost btn-sm text-base-content/60 inline-flex"
      >
        返回目录
      </a>

      {#if data.next_id}
        <a
          href={`/${page.params.novelId}/${data.next_id}`}
          class="btn btn-primary sm:w-32"
        >
          下一章 »
        </a>
      {:else}
        <button
          class="btn btn-primary btn-disabled sm:w-32"
          aria-disabled="true"
        >
          已是最后一章
        </button>
      {/if}
    </nav>
  </article>
</div>
