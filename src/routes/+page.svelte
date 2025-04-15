<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { Button } from '$lib/components/ui/button'
  import * as Card from '$lib/components/ui/card'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'

  let isLoading = $state(false)
  let mailConfig: Promise<string> | null = $state(null)

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
        <Card.Description>
          Sign up using on of the following providers (I know, so many options):
        </Card.Description>
      </Card.Header>
      <Card.Content class="grid gap-4">
        <Button variant="outline" onclick={handleLoginWithGoogle}>
          Google
        </Button>
      </Card.Content>
      <Card.Footer></Card.Footer>
    </Card.Root>
  {/if}
</div>
