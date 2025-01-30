<script>
  import { superForm, defaults } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import LoaderCircle from 'lucide-svelte/icons/loader-circle';
  import { jwtDecode } from 'jwt-decode';
  import { toast } from 'svelte-sonner';

  import { goto } from '$app/navigation';
  import { cfetch } from '$lib/utils.js';
  import { auth } from '$lib/state/auth.svelte.js';
  import { loginSchema } from '$lib/schema';
  import AuthAlert from '$lib/components/auth-alert.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Button } from '$lib/components/ui/button/index.js';

  /** @type {boolean} */
  let loading = $state(false);

  /**
   * @typedef {Object} Alert
   * @property {string} title
   * @property {string} message
   * @property {boolean} error
   */
  /** @type {Alert | undefined} */
  let msg = $state();

  const form = superForm(defaults(zod(loginSchema)), {
    SPA: true,
    validators: zod(loginSchema),
    multipleSubmits: 'prevent',
    async onUpdate({ form }) {
      if (!form.valid) return;
      msg = undefined;
      loading = true;
      try {
        const res = await cfetch('/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify($formData)
        });
        console.log(res);
        // console.log(res);
        if (res.ok) {
          let d = await res.json();
          console.log(jwtDecode(d.token));
          auth.token = d.token;
          let ures = await cfetch('/user', { credentials: 'same-origin' });
          if (ures.ok) {
            let data = await ures.json();
            auth.user = data.user;
            goto('/');
          } else {
            let data = await ures.json();
            msg = {
              title: 'Error',
              message: data.error,
              error: true
            };
          }
        } else {
          let data = await res.json();
          msg = {
            title: 'Error',
            message: data.error,
            error: true
          };
        }
      } catch (e) {
        // console.log(e);
        toast.error('Network error', {
          description:
            'Failed to connect to server. Please check your internet connection and try again.'
        });
        msg = {
          title: 'Error',
          message: `${e}`,
          error: true
        };
      } finally {
        $formData.password = '';
        loading = false;
      }
    }
  });

  const { form: formData, enhance, capture, restore, errors } = form;

  export const snapshot = { capture, restore };
</script>

<svelte:head>
  <title>Login</title>
  <meta name="description" content="Login to the system" />
</svelte:head>

<form method="POST" use:enhance>
  <Card.Root class="m-auto mt-20 w-[350px]">
    <Card.Header>
      <Card.Title>Login</Card.Title>
      <Card.Description>Login to the system.</Card.Description>
    </Card.Header>
    <Card.Content>
      {#if msg}
        <AuthAlert {...msg} />
      {/if}
      <Form.Field {form} name="username">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Username</Form.Label>
            <Input {...props} placeholder="Enter your username.." bind:value={$formData.username} />
          {/snippet}
        </Form.Control>
        <!-- <Form.Description>This is your public display name.</Form.Description> -->
        <Form.FieldErrors />
      </Form.Field>
      <Form.Field {form} name="password">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Password</Form.Label>
            <Input
              {...props}
              type="password"
              placeholder="Enter your password.."
              bind:value={$formData.password}
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
    </Card.Content>
    <Card.Footer class="flex justify-between">
      <Form.Button disabled={$errors.username || $errors.password || loading}>Login</Form.Button>
      {#if loading}
        <Button disabled variant="ghost">
          <LoaderCircle class="animate-spin" />
          Please wait
        </Button>
      {/if}
    </Card.Footer>
  </Card.Root>
</form>
