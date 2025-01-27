<script>
  import { page } from '$app/state';
  import { toggleMode } from 'mode-watcher';
  import Sun from 'lucide-svelte/icons/sun';
  import Moon from 'lucide-svelte/icons/moon';

  import { Trigger } from '$lib/components/ui/sidebar/index.js';
  import * as Avatar from '$lib/components/ui/avatar/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
    import { goto } from '$app/navigation';
</script>

<header
  class="flex h-16 shrink-0 justify-between transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
  style="view-transition-name: header;"
>
  <div class="sticky flex items-center gap-2 px-4">
    <span class="flex items-center justify-center">
      <Trigger />
    </span>
  </div>

  <nav>
    <svg viewBox="0 0 2 3" aria-hidden="true">
      <path d="M0,0 L1,2 C1.5,3 1.5,3 2,3 L2,0 Z" />
    </svg>
    <ul>
      <li aria-current={page.url.pathname.startsWith('/register') ? 'page' : undefined}>
        <a href="/register">Borrow</a>
      </li>
      <li aria-current={page.url.pathname.startsWith('/login') ? 'page' : undefined}>
        <a href="/login">Return</a>
      </li>
    </ul>
    <svg viewBox="0 0 2 3" aria-hidden="true">
      <path d="M0,0 L0,3 C0.5,3 0.5,3 1,2 L2,0 Z" />
    </svg>
  </nav>

  <div class="flex items-center gap-2 px-4">
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        <Avatar.Root class="h-8 w-8">
          <Avatar.Image src="https://github.com/krivahtoo.png" alt="@shadcn" />
          <Avatar.Fallback>SC</Avatar.Fallback>
        </Avatar.Root>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content class="w-56" align="end">
        <DropdownMenu.Label class="font-normal">
          <div class="flex flex-col space-y-1">
            <p class="text-sm font-medium leading-none">krivah</p>
            <p class="text-xs leading-none text-muted-foreground">krivahtoo@github.com</p>
          </div>
        </DropdownMenu.Label>
        <DropdownMenu.Separator />
        <DropdownMenu.Group>
          <DropdownMenu.Item>
            Profile
            <DropdownMenu.Shortcut>⇧⌘P</DropdownMenu.Shortcut>
          </DropdownMenu.Item>
          <DropdownMenu.Item>
            Billing
            <DropdownMenu.Shortcut>⌘B</DropdownMenu.Shortcut>
          </DropdownMenu.Item>
          <DropdownMenu.Item>
            Settings
            <DropdownMenu.Shortcut>⌘S</DropdownMenu.Shortcut>
          </DropdownMenu.Item>
          <DropdownMenu.Item onclick={toggleMode}>
            Switch Mode
            <DropdownMenu.Shortcut>
              <Sun class="rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
              <Moon
                class="absolute top-2 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
              />
            </DropdownMenu.Shortcut>
          </DropdownMenu.Item>
        </DropdownMenu.Group>
        <DropdownMenu.Separator />
        <DropdownMenu.Item onclick={() => goto('/login')}>
          Log out
          <DropdownMenu.Shortcut>⇧⌘Q</DropdownMenu.Shortcut>
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </div>
</header>

<style>
  nav {
    display: flex;
    justify-content: center;
    --background: hsl(var(--card) / 0.9);
  }

  svg {
    width: 2em;
    height: 3em;
    display: block;
  }

  path {
    fill: var(--background);
  }

  ul {
    position: relative;
    padding: 0;
    margin: 0;
    height: 3em;
    display: flex;
    justify-content: center;
    align-items: center;
    list-style: none;
    background: var(--background);
    background-size: contain;
  }

  li {
    position: relative;
    height: 100%;
  }

  li[aria-current='page']::before {
    --size: 6px;
    content: '';
    width: 0;
    height: 0;
    position: absolute;
    top: 0;
    left: calc(50% - var(--size));
    border: var(--size) solid transparent;
    border-top: var(--size) solid #17a24a;
    view-transition-name: active-page;
  }

  nav a {
    display: flex;
    height: 100%;
    align-items: center;
    padding: 0 0.5rem;
    color: var(--primary);
    font-weight: 700;
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    text-decoration: none;
    transition: color 0.2s linear;
  }
</style>
