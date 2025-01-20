<script>
  import { Skeleton } from '$lib/components/ui/skeleton/index.js';
</script>

<svelte:head>
  <title>Maktaba</title>
  <meta name="description" content="Library management system" />
</svelte:head>

<div class="flex flex-1 flex-col gap-4 p-4 pt-0">
  <div class="grid auto-rows-min gap-4 md:grid-cols-3">
    <Skeleton class="aspect-video rounded-xl bg-muted/90" />
    <Skeleton class="aspect-video rounded-xl bg-muted/90" />
    <Skeleton class="aspect-video rounded-xl bg-muted/90" />
  </div>
  <div class="min-h-[100vh] flex-1 rounded-xl bg-muted/90 md:min-h-min"></div>
</div>
