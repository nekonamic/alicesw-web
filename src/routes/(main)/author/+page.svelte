<script lang="ts">
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { PUBLIC_PAGE_SIZE } from "$env/static/public";
  const pageSize = Number(PUBLIC_PAGE_SIZE);

  let { data } = $props();
  const kw = $derived(page.url.searchParams.get("kw"));
  const currentPage = $derived(page.url.searchParams.get("page"));

  let jumpPage = $state<number>(1);

  $effect(() => {
    if (currentPage) {
      jumpPage = Number(currentPage);
    }
  });

  async function handlePage(newPage: number) {
    const url = new URL(window.location.href);
    url.searchParams.set("page", newPage.toString());
    await goto(url.pathname + url.search);
  }
</script>

<div class="container mx-auto p-6">
  <h1 class="text-3xl font-bold mb-8">作者 {kw}:</h1>

  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    {#each data.result.results as item (item.novelId)}
      <div
        class="card bg-base-200 shadow-lg hover:shadow-xl transition-shadow duration-300"
      >
        <div class="card-body">
          <h2 class="card-title text-lg line-clamp-2">
            {item.novelTitle}
          </h2>

          <p class="text-sm text-base-content/70 line-clamp-2">
            {item.author}
          </p>

          <div class="card-actions justify-end">
            <a
              class="btn btn-sm btn-secondary"
              href="/author?kw={item.author}&page=1">搜索作者</a
            >
            <a class="btn btn-sm btn-primary" href="/{item.novelId}">查看小说</a
            >
          </div>
        </div>
      </div>
    {/each}
  </div>

  <div class="flex justify-center items-center mt-6">
    <div class="join">
      {#if currentPage === "1"}
        <button class="join-item btn btn-lg btn-disabled">«</button>
      {:else}
        <button
          class="join-item btn btn-lg"
          onclick={() => {
            handlePage(Number(currentPage) - 1);
          }}>«</button
        >
      {/if}
      <div class="dropdown dropdown-top dropdown-center">
        <div tabindex="0" role="button" class="join-item btn btn-lg">
          {currentPage}/{Math.ceil(data.result.total / pageSize)}
        </div>
        <div class="dropdown-content bg-base-100 rounded-box p-4 shadow-sm">
          <form
            onsubmit={(e) => {
              e.preventDefault();
              handlePage(jumpPage);
            }}
            class="flex flex-row"
          >
            <button
              type="submit"
              class="btn flex-1 whitespace-nowrap rounded-l-md">跳转</button
            >
            <label class="input rounded-r-md">
              <span class="label-text">第</span>
              <input
                type="text"
                class="w-24"
                placeholder="1-{Math.ceil(data.result.total / pageSize)}"
                bind:value={jumpPage}
              />
              <span class="label-text">页</span>
            </label>
          </form>
        </div>
      </div>
      {#if Number(currentPage) * pageSize > data.result.total || data.result.total < pageSize}
        <button class="join-item btn btn-lg btn-disabled">»</button>
      {:else}
        <button
          class="join-item btn btn-lg"
          onclick={() => {
            handlePage(Number(currentPage) + 1);
          }}>»</button
        >
      {/if}
    </div>
  </div>
</div>
