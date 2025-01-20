<script>
  import { superForm, defaults, fileProxy } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import LoaderCircle from 'lucide-svelte/icons/loader-circle';

  import { goto } from '$app/navigation';
  import { registerSchema } from '$lib/schema';
  import AuthAlert from '$lib/components/auth-alert.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input, FileInput } from '$lib/components/ui/input/index.js';
  import { Button } from '$lib/components/ui/button/index.js';

  let loading = $state(false);

  /**
   * @typedef {Object} Alert
   * @property {string} title
   * @property {string} message
   * @property {boolean} error
   */
  /** @type {Alert | undefined} */
  let msg = $state();

  const form = superForm(defaults(zod(registerSchema)), {
    SPA: true,
    validators: zod(registerSchema),
    multipleSubmits: 'prevent',
    onUpdate({ form }) {
      console.log(form);
      // Form validation
      if (!form.valid) return;
      msg = undefined;
      loading = true;
      // TODO: Call the API with form.data, await the result and update form
      return setTimeout(() => {
        msg = {
          title: 'Success',
          message: 'You are now registered',
          error: false
        };
        loading = false;
        goto('/login');
      }, 2000);
    }
  });

  const { form: formData, enhance, capture, restore, errors } = form;

  const file = fileProxy(form, 'image');

  export const snapshot = { capture, restore };
</script>

<svelte:head>
  <title>Login</title>
  <meta name="description" content="Login to the system" />
</svelte:head>

<form method="POST" enctype="multipart/form-data" use:enhance>
  <Card.Root class="m-auto mt-20 md:w-2/3 lg:w-1/2">
    <Card.Header class="">
      <Card.Title class="">Register</Card.Title>
      <Card.Description class="">Register to the system.</Card.Description>
    </Card.Header>
    <Card.Content class="">
      {#if msg}
        <AuthAlert {...msg} />
      {/if}
      <div class="flex flex-col md:flex-row md:space-x-2">
        <Form.Field {form} name="name">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label>Name</Form.Label>
              <Input {...props} bind:value={$formData.name} placeholder="Enter your name..." />
            {/snippet}
          </Form.Control>
          <!-- <Form.Description>This is your public display name.</Form.Description> -->
          <Form.FieldErrors />
        </Form.Field>
        <Form.Field {form} name="username">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label>Username</Form.Label>
              <Input
                {...props}
                bind:value={$formData.username}
                placeholder="Enter your username..."
              />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>
      </div>
      <Form.Field {form} name="image">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Profile Photo</Form.Label>
            <FileInput {...props} type="file" bind:files={$file} />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
      <Form.Field {form} name="password">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label class="">Password</Form.Label>
            <Input
              {...props}
              type="password"
              bind:value={$formData.password}
              placeholder="Enter your password..."
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
      <Form.Field {form} name="confirm">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Confirm</Form.Label>
            <Input
              {...props}
              type="password"
              bind:value={$formData.confirm}
              placeholder="Confirm your password..."
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
    </Card.Content>
    <Card.Footer class="flex justify-between">
      <Form.Button disabled={$errors.username || $errors.password || $errors.confirm || loading}
        >Submit</Form.Button
      >
      {#if loading}
        <Button disabled variant="ghost">
          <LoaderCircle class="animate-spin" />
          Please wait
        </Button>
      {/if}
    </Card.Footer>
  </Card.Root>
</form>
