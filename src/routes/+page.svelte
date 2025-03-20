<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { Button } from '$lib/components/ui/button'
  import * as Card from '$lib/components/ui/card'
  import { Label } from '$lib/components/ui/label'
  import { Input } from '$lib/components/ui/input'
  import { getDomainFromMail } from '$lib/utils'

  let mail = $state('')
  let mailConfig: Promise<string> | null = $state(null)

  const handleEmailEntered = (event: Event) => {
    let domain = getDomainFromMail(mail)
    mailConfig = invoke('get_mail_config', { domain })
  }
</script>

<div class="flex items-center justify-center h-screen">
  <Card.Root class="max-w-sm mx-auto">
    <Card.Header class="space-y-1">
      <Card.Title class="text-2xl">Add Account</Card.Title>
      <Card.Description>Enter your Email Address to continue.</Card.Description>
    </Card.Header>
    <Card.Content class="grid gap-4">
      <div class="grid gap-2">
        <Label for="email">Email</Label>
        <Input
          bind:value={mail}
          id="email"
          type="email"
          placeholder="m@example.com"
        />
        {#if mailConfig === null}
          <Button onclick={handleEmailEntered} class="w-full">Continue</Button>
        {/if}
      </div>
      {#if mailConfig !== null}
        <div class="grid gap-2">
          {#await mailConfig}
            <p>Loading...</p>
          {:then config}
            <span>{config}</span>
            <Button class="w-full">Add Account</Button>
          {:catch error}
            <p class="text-red-500">
              Getting the mail-server configuration failed: {error.message}
            </p>
          {/await}
        </div>
      {/if}
    </Card.Content>
    <Card.Footer></Card.Footer>
  </Card.Root>
</div>
