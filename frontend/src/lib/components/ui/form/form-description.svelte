<script>
  import * as FormPrimitive from 'formsnap';
  import { cn } from '$lib/utils.js';

  let { ref = $bindable(null), class: className = '', ...restProps } = $props();
</script>

<FormPrimitive.Description
  bind:ref
  class={cn('text-sm text-muted-foreground', className)}
  {...restProps}
/>
