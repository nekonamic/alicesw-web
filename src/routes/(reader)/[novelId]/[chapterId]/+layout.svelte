<script lang="ts">
import { onMount } from "svelte";
import favicon from "$lib/assets/favicon.svg";
import { favorites } from "$lib/utils/favorites.svelte";
  import { page } from "$app/state";

let { data, children } = $props();
let prevScrollPos = $state(0);
let navbar: HTMLElement;

onMount(() => {
	navbar = document.getElementById("navbar")!;
	prevScrollPos = window.pageYOffset;

	window.onscroll = () => {
		let currentScrollPos = window.pageYOffset;
		if (prevScrollPos > currentScrollPos) {
			navbar.style.transform = "translateY(0)";
		} else {
			navbar.style.transform = "translateY(-100%)";
		}
		prevScrollPos = currentScrollPos;
	};
});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<div
  class="navbar bg-base-200 fixed w-full transition-transform duration-300 transform-gpu z-50"
  id="navbar"
>
  <div class="flex-1 flex flex-row items-center sm:gap-2">
    <div class="hover-3d ml-2">
      <!-- content -->
      <figure class="max-w-100 rounded-2xl">
        <img src={favicon} alt="Favicon" width="32" height="32" />
      </figure>
      <!-- 8 empty divs needed for the 3D effect -->
      <div></div>
      <div></div>
      <div></div>
      <div></div>
      <div></div>
      <div></div>
      <div></div>
      <div></div>
    </div>
    <a class="btn btn-ghost text-xl" href="/">魔理沙书屋</a>
  </div>
  <div class="flex-none flex flex-row items-center gap-2">
    <div class="drawer">
      <input id="my-drawer-1" type="checkbox" class="drawer-toggle" />
      <div class="drawer-content">
        <!-- Page content here -->
        <label for="my-drawer-1" class="btn btn-ghost drawer-button">
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
              d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z"
            />
          </svg>
        </label>
      </div>
      <div class="drawer-side">
        <label
          for="my-drawer-1"
          aria-label="close sidebar"
          class="drawer-overlay"
        ></label>
        <ul class="menu bg-base-200 min-h-full w-80 p-4">
          <!-- Sidebar content here -->

          <ul class="list bg-base-100 rounded-box shadow-md p-4">
            <a class="pb-2 text-xl font-bold tracking-wide pl-2" href="/{data.result.id}" aria-label="打开小说" 
            onclick={() => {
              const drawer = document.getElementById('my-drawer-1') as HTMLInputElement;
              drawer.checked = false;
            }}>
              {data.result.title}
            </a>
            <div class="flex flex-col gap-1">
              {#each data.result.chapterInfo as chapterInfo}
              {@const isCurrent = chapterInfo.id === page.params.chapterId}
                <a
                  class="bg-base-100 rounded-lg p-2
                  {isCurrent 
                    ? 'border-primary bg-primary/5 transition-all' 
                    : 'hover:border-primary hover:bg-primary/5 transition-all'}"
                  href="/{data.result.id}/{chapterInfo.id}"
                  aria-label="打开章节"
                  onclick={() => {
                    const drawer = document.getElementById('my-drawer-1') as HTMLInputElement;
                    drawer.checked = false;
                  }}
                >
                  {chapterInfo.title}
                </a>
              {/each}
            </div>
            <dir class="mb-2"></dir>
          </ul>
        </ul>
      </div>
    </div>
    <div class="drawer">
      <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
      <div class="drawer-content">
        <!-- Page content here -->
        <label for="my-drawer-2" class="btn btn-ghost drawer-button">
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
              d="M16.5 3.75V16.5L12 14.25 7.5 16.5V3.75m9 0H18A2.25 2.25 0 0 1 20.25 6v12A2.25 2.25 0 0 1 18 20.25H6A2.25 2.25 0 0 1 3.75 18V6A2.25 2.25 0 0 1 6 3.75h1.5m9 0h-9"
            />
          </svg>
        </label>
      </div>
      <div class="drawer-side">
        <label
          for="my-drawer-2"
          aria-label="close sidebar"
          class="drawer-overlay"
        ></label>
        <ul class="menu bg-base-200 min-h-full w-80 p-4">
          <!-- Sidebar content here -->

          <ul class="list bg-base-100 rounded-box shadow-md">
            <p class="p-4 pb-2 text-lg tracking-wide">收藏夹</p>

            {#each favorites.items as favorite}
              <div class="flex flex-row items-center justify-between px-4">
                <div class="font-semibold">
                  {favorite.title}
                </div>
                <div>
                  <a
                    class="btn btn-square btn-ghost"
                    href="/{favorite.id}"
                    aria-label="阅读"
                    onclick={() => {
                      const drawer = document.getElementById('my-drawer-2') as HTMLInputElement;
                      drawer.checked = false;
                    }}
                  >
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
                        d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z"
                      />
                    </svg>
                  </a>
                </div>
              </div>
            {/each}
            {#if favorites.isEmpty}
              <p class="text-xl font-bold opacity-50 mx-auto">空</p>
            {/if}
            <dir class="mb-2"></dir>
          </ul>
        </ul>
      </div>
    </div>
  </div>
</div>
<div class="h-16"></div>
{@render children()}
