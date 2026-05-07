<script lang="ts">
import "./layout.css";
import favicon from "$lib/assets/favicon.svg";

let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<div class="navbar bg-base-200">
    <div class="navbar-start gap-2">
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
        <div class="flex-1">
            <a class="btn btn-ghost text-xl" href="/">魔理沙书屋</a>
        </div>
    </div>
    <div class="navbar-center"></div>
    <div class="navbar-end"></div>
</div>
{@render children()}
