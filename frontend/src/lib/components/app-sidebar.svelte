<script>
  import House from 'lucide-svelte/icons/house';
  import Search from 'lucide-svelte/icons/search';
  import Settings from 'lucide-svelte/icons/settings';
  import UsersRound from 'lucide-svelte/icons/users-round';
  import Activity from 'lucide-svelte/icons/activity';
  import LibraryBig from 'lucide-svelte/icons/library-big';
  import Ellipsis from 'lucide-svelte/icons/ellipsis';

  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

  // Menu items.
  const items = [
    {
      title: 'Home',
      url: '#',
      icon: House
    },
    {
      title: 'Search',
      url: '#',
      icon: Search
    },
    {
      title: 'Borowing',
      url: '#',
      icon: Activity
    },
    {
      title: 'Books',
      url: '#',
      icon: LibraryBig
    },
    {
      title: 'Users',
      url: '#',
      icon: UsersRound
    }
  ];
</script>

<Sidebar.Root variant="floating" collapsible="icon">
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupLabel>Application</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each items as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                {#snippet child({ props })}
                  <a href={item.url} {...props}>
                    <Tooltip.Provider>
                      <Tooltip.Root delayDuration={200}>
                        <Tooltip.Trigger>
                          <item.icon class="h-4 w-4" />
                        </Tooltip.Trigger>
                        <Tooltip.Content side="right">
                          <p>{item.title}</p>
                        </Tooltip.Content>
                      </Tooltip.Root>
                    </Tooltip.Provider>
                    <span>{item.title}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Footer>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton
          class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
        >
          {#snippet child({ props })}
            <a href="/" {...props}>
              <Tooltip.Provider>
                <Tooltip.Root delayDuration={200}>
                  <Tooltip.Trigger>
                    <Settings class="h-4 w-4" />
                  </Tooltip.Trigger>
                  <Tooltip.Content side="right">
                    <p>Settings</p>
                  </Tooltip.Content>
                </Tooltip.Root>
              </Tooltip.Provider>
              <span>Settings</span>
            </a>
          {/snippet}
        </Sidebar.MenuButton>
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            {#snippet child({ props })}
              <Sidebar.MenuAction {...props}>
                <Ellipsis />
              </Sidebar.MenuAction>
            {/snippet}
          </DropdownMenu.Trigger>
          <DropdownMenu.Content side="right" align="end">
            <DropdownMenu.Group>
              <DropdownMenu.Item>Profile</DropdownMenu.Item>
              <DropdownMenu.Item>
                <a href="/about">About</a>
              </DropdownMenu.Item>
            </DropdownMenu.Group>
            <DropdownMenu.Separator />
            <DropdownMenu.Item>Log out</DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Footer>
</Sidebar.Root>
