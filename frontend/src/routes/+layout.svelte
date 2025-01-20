<script>
  import { Toaster } from 'svelte-sonner';
  import { ModeWatcher } from 'mode-watcher';

  import Header from './Header.svelte';
  import '../app.css';
  import { onNavigate } from '$app/navigation';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import AppSidebar from '$lib/components/app-sidebar.svelte';

  onNavigate((navigation) => {
    if (!document.startViewTransition) return;

    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
    });
  });

  /** @type {{children: import('svelte').Snippet}} */
  let { children } = $props();

  // let open = $state(false);
</script>

<ModeWatcher />

<Toaster richColors closeButton />

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
  @keyframes fade-in {
    from {
      opacity: 0;
    }
  }

  @keyframes fade-out {
    to {
      opacity: 0;
    }
  }

  @keyframes slide-from-right {
    from {
      transform: translateX(30px);
    }
  }

  @keyframes slide-to-left {
    to {
      transform: translateX(-30px);
    }
  }

  :root::view-transition-old(main) {
    animation:
      90ms cubic-bezier(0.4, 0, 1, 1) both fade-out,
      300ms cubic-bezier(0.4, 0, 0.2, 1) both slide-to-left;
  }

  :root::view-transition-new(main) {
    animation:
      210ms cubic-bezier(0, 0, 0.2, 1) 90ms both fade-in,
      300ms cubic-bezier(0.4, 0, 0.2, 1) both slide-from-right;
  }
</style>
