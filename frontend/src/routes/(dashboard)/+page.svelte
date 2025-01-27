<script>
  import DollarSign from 'lucide-svelte/icons/dollar-sign';

  import { Skeleton } from '$lib/components/ui/skeleton/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
</script>

<svelte:head>
  <title>Maktaba</title>
  <meta name="description" content="Library management system" />
</svelte:head>

<div class="flex flex-1 flex-col gap-4 p-4 pt-0">
  <div class="grid auto-rows-min gap-4 md:grid-cols-3">
    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Total Books</Card.Title>
        <DollarSign class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">5,233</div>
        <p class="mt-3 text-xs text-muted-foreground">201 are curently borrowed</p>
      </Card.Content>
    </Card.Root>
    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Total Members</Card.Title>
        <DollarSign class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">467</div>
        <p class="mt-3 text-xs text-muted-foreground">5 are staff</p>
      </Card.Content>
    </Card.Root>
    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Total Members</Card.Title>
        <DollarSign class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">5,233</div>
        <p class="mt-3 text-xs text-muted-foreground">201 are curently borrowed</p>
      </Card.Content>
    </Card.Root>
  </div>

    <Card.Root class="col-span-3">
      <Card.Header>
        <Card.Title>Recent Sales</Card.Title>
        <Card.Description>You made 265 sales this month.</Card.Description>
      </Card.Header>
      <Card.Content>
        <div class="space-y-8">
          <div class="flex items-center">
            <!-- <Avatar.Root class="h-9 w-9"> -->
            <!-- 	<Avatar.Image src="/avatars/01.png" alt="Avatar" /> -->
            <!-- 	<Avatar.Fallback>OM</Avatar.Fallback> -->
            <!-- </Avatar.Root> -->
            <div class="ml-4 space-y-1">
              <p class="text-sm font-medium leading-none">Olivia Martin</p>
              <p class="text-sm text-muted-foreground">olivia.martin@email.com</p>
            </div>
            <div class="ml-auto font-medium">+$1,999.00</div>
          </div>
          <div class="flex items-center">
            <div class="ml-4 space-y-1">
              <p class="text-sm font-medium leading-none">Jackson Lee</p>
              <p class="text-sm text-muted-foreground">jackson.lee@email.com</p>
            </div>
            <div class="ml-auto font-medium">+$39.00</div>
          </div>
          <div class="flex items-center">
            <div class="ml-4 space-y-1">
              <p class="text-sm font-medium leading-none">Isabella Nguyen</p>
              <p class="text-sm text-muted-foreground">isabella.nguyen@email.com</p>
            </div>
            <div class="ml-auto font-medium">+$299.00</div>
          </div>
          <div class="flex items-center">
            <div class="ml-4 space-y-1">
              <p class="text-sm font-medium leading-none">William Kim</p>
              <p class="text-sm text-muted-foreground">will@email.com</p>
            </div>
            <div class="ml-auto font-medium">+$99.00</div>
          </div>
          <div class="flex items-center">
            <div class="ml-4 space-y-1">
              <p class="text-sm font-medium leading-none">Sofia Davis</p>
              <p class="text-sm text-muted-foreground">sofia.davis@email.com</p>
            </div>
            <div class="ml-auto font-medium">+$39.00</div>
          </div>
        </div>
      </Card.Content>
    </Card.Root>

</div>
