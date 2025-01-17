<script>
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import BadgeCheck from 'lucide-svelte/icons/badge-check';

  import * as Alert from '$lib/components/ui/alert/index.js';

  const { title, message, error = false } = $props();
</script>

<Alert.Root variant={error ? 'destructive' : 'primary'}>
  {#if error}
    <CircleAlert class="size-4" />
  {:else}
    <BadgeCheck class="size-4" />
  {/if}
  <Alert.Title>{title}</Alert.Title>
  <Alert.Description>{message}</Alert.Description>
</Alert.Root>
