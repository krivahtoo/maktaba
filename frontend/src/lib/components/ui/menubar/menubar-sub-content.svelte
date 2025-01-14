<script>
	import { Menubar as MenubarPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	let { ref = $bindable(null), class: className, ...restProps } = $props();
</script>

<MenubarPrimitive.SubContent
	bind:ref
	class={cn(
		'z-50 min-w-max rounded-md border bg-popover p-1 text-popover-foreground focus:outline-none',
		className
	)}
	{...restProps}
/>
