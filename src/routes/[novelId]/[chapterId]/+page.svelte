<script lang="ts">
  import { page } from '$app/state';

  let novelId = $derived(page.params.novelId);
  let chapterId = $derived(page.params.chapterId);
</script>

<p>novel: {novelId}</p>
<p>chapter: {chapterId}</p>