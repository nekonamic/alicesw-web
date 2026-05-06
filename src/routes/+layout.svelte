<script lang="ts">
  import "./layout.css";
  import favicon from "$lib/assets/favicon.svg";
  import ThemeSwitch from "$lib/components/ThemeSwitch.svelte";

  let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<div class="navbar bg-base-100">
  <div class="navbar-start"></div>
  <div class="navbar-center"></div>
  <div class="navbar-end">
    <ThemeSwitch />
  </div>
</div>
{@render children()}
