<script lang="ts">
import Logo from "$lib/components/Logo.svelte";
import { goto } from "$app/navigation";

type SearchTarget = "content" | "title" | "author";

const options: { value: SearchTarget; label: string }[] = [
	{ value: "content", label: "内容" },
	{ value: "title", label: "书名" },
	{ value: "author", label: "作者" },
];

let selected = $state<SearchTarget>("content");
let keyword = $state("");

async function handleSearch() {
	if (!keyword.trim()) {
		return;
	}

	const params = new URLSearchParams({
		kw: keyword,
		page: "1",
	});

	switch (selected) {
		case "content":
			await goto(`/content?${params.toString()}`);
			break;
		case "title":
			await goto(`/title?${params.toString()}`);
			break;
		case "author":
			await goto(`/author?${params.toString()}`);
			break;
	}
}
</script>

<div class="flex items-center flex-col">
  <Logo />
  <form
    class="join w-full md:w-auto px-4 md:px-0"
    onsubmit={(e) => {
      e.preventDefault();
      handleSearch();
    }}
  >
    <input
      class="input join-item rounded-l-lg w-full md:w-100"
      placeholder="请输入..."
      type="search"
      enterkeyhint="search"
      bind:value={keyword}
      onkeydown={(e) => {
        if (e.key === "Enter") {
          e.preventDefault();
          handleSearch();
        }
      }}
      onkeyup={(e) => {
        if (e.key === "Enter") {
          e.preventDefault();
          handleSearch();
        }
      }}
    />
    <select class="select join-item w-24" bind:value={selected}>
      {#each options as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
    <button
      type="submit"
      class="btn join-item rounded-r-lg text-sm md:text-base"
    >
      搜索
    </button>
  </form>

  <div class="mt-2 space-x-1">
    <a href="/info" class="btn">使用说明</a>
    <a href="/random" class="btn">随机小说</a>
  </div>
</div>
