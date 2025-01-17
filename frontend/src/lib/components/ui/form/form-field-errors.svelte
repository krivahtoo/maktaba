<script>
  import * as FormPrimitive from 'formsnap';
  import { cn } from '$lib/utils.js';

  let {
    ref = $bindable(null),
    class: className = '',
    errorClasses = '',
    children: childrenProp = undefined,
    ...restProps
  } = $props();
</script>

<FormPrimitive.FieldErrors
  bind:ref
  class={cn('text-xs font-thin text-destructive', className)}
  {...restProps}
>
  {#snippet children({ errors, errorProps })}
    {#if childrenProp}
      {@render childrenProp({ errors, errorProps })}
    {:else}
      {#each errors as error}
        {#if Array.isArray(error)}
          {#each error as er}
            <div {...errorProps} class={cn(errorClasses)}>{er}</div>
          {/each}
        {:else}
          <div {...errorProps} class={cn(errorClasses)}>{error}</div>
        {/if}
      {/each}
    {/if}
  {/snippet}
</FormPrimitive.FieldErrors>
