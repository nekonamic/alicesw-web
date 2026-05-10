<script lang="ts">
  let { data } = $props();
</script>

<div class="container mx-auto p-6">
  <h1 class="text-3xl font-bold mb-8">随机小说</h1>

  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    {#each data.result as item (item.id)}
      <div
        class="card bg-base-200 shadow-lg hover:shadow-xl transition-shadow duration-300"
      >
        <div class="card-body">
          <h2 class="card-title text-lg line-clamp-2">
            {item.title}
          </h2>

          <p class="text-sm text-base-content/70 line-clamp-2">
            {item.author}
          </p>

          <div class="card-actions justify-end">
            <a
              class="btn btn-sm btn-secondary"
              href="/author?kw={item.author}&page=1">搜索作者</a
            >
            <a class="btn btn-sm btn-primary" href="/{item.id}">查看小说</a>
          </div>
        </div>
      </div>
    {/each}
  </div>

  <div class="flex justify-center mt-4">
    <a class="btn btn-lg" href="/random" data-sveltekit-reload>再来一次</a>
  </div>
</div>
