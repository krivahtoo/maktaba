<script>
  import Header from '$lib/components/app-header.svelte';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import AppSidebar from '$lib/components/app-sidebar.svelte';

  /** @type {{children: import('svelte').Snippet}} */
  let { children } = $props();

  // let open = $state(false);
</script>

<Sidebar.Provider>
  <AppSidebar />
  <Sidebar.Inset class="min-w-screen flex min-h-screen flex-col bg-transparent">
    <Header />

    <main class="">
      {@render children()}
    </main>

    <footer class="text-xs">
      <p>
        Made with ❤️ by <a href="https://github.com/krivahtoo">Noah Too</a>
      </p>
    </footer>
  </Sidebar.Inset>
</Sidebar.Provider>

<style>
  main {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 1rem;
    width: 100%;
    max-width: 64rem;
    margin: 0 auto;
    box-sizing: border-box;
    view-transition-name: main;
  }

  footer {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 12px;
  }

  footer a {
    font-weight: bold;
  }

  @media (min-width: 480px) {
    footer {
      padding: 12px 0;
    }
  }
</style>
