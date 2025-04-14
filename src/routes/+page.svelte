<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { Button } from '$lib/components/ui/button'
  import * as Card from '$lib/components/ui/card'
  import { Label } from '$lib/components/ui/label'
  import { Input } from '$lib/components/ui/input'
  import { getDomainFromMail, sleep } from '$lib/utils'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'

  let mail = $state('')
  let isLoading = $state(false)
  let mailConfig: Promise<string> | null = $state(null)

  const handleEmailEntered = async (event: Event) => {
    isLoading = true
    let domain = getDomainFromMail(mail)
    await sleep(2000)
    mailConfig = invoke('get_mail_config', { domain })
    isLoading = false
  }

  const handleLoginWithGoogle = async (event: Event) => {
    isLoading = true
    mailConfig = invoke('login_with_google')
    mailConfig.then(() => (isLoading = false))
  }
</script>

<div class="flex items-center justify-center h-screen">
  {#if isLoading}
    <div class="absolute inset-0 flex items-center justify-center">
      <LoadingSpinner
        color="#FF3E00"
        size="50"
        unit="px"
        duration="4s"
        stroke="5"
        pause={!isLoading}
      />
    </div>
  {:else}
    <Card.Root class="max-w-sm mx-auto">
      <Card.Header class="space-y-1">
        <Card.Title class="text-2xl">Add Account</Card.Title>
        <Card.Description
          >Enter your Email Address to continue.</Card.Description
        >
      </Card.Header>
      <Card.Content class="grid gap-4">
        <Button variant="outline" onclick={handleLoginWithGoogle}
          >Login with Google</Button
        >
        <div class="relative">
          <div class="absolute inset-0 flex items-center">
            <span class="w-full border-t"></span>
          </div>
          <div class="relative flex justify-center text-xs uppercase">
            <span class="bg-card text-muted-foreground px-2">
              Or continue with
            </span>
          </div>
        </div>
        <div class="grid gap-2">
          <Label for="email">Email</Label>
          <Input
            bind:value={mail}
            id="email"
            type="email"
            placeholder="m@example.com"
          />
          {#if mailConfig === null}
            <Button onclick={handleEmailEntered} class="w-full">Continue</Button
            >
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
  {/if}
</div>
