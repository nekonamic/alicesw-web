<script>
  import "./layout.css";
  import Logo from "$lib/components/Logo.svelte";

  const options = [
    { value: "content", label: "内容" },
    { value: "title", label: "书名" },
    { value: "author", label: "作者" },
  ];

  let selected = $state("content");
</script>

<div class="flex items-center justify-center flex-col">
  <Logo />
  <div class="join">
    <input class="input join-item rounded-l-lg w-100" placeholder="Search" />
    <select class="select join-item w-20" bind:value={selected}>
      {#each options as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
    <button class="btn join-item rounded-r-lg">Search</button>
  </div>
</div>
