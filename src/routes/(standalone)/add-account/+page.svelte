<script lang="ts">
  import { Button } from '$lib/components/ui/button'
  import * as Card from '$lib/components/ui/card'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'
  import Google from '$lib/components/custom/Google.svelte'
  import Input from '$lib/components/ui/input/input.svelte'
  import { loginWithGoogle } from '$lib/commands'

  let isLoading = $state(false)
  let email: string = $state('')

  const handleLoginWithGoogle = async (event: Event) => {
    isLoading = true
    await loginWithGoogle(email)
    isLoading = false
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
        <Input
          bind:value={email}
          type="text"
          placeholder="Email"
          class="w-full"
          disabled={isLoading}
          onkeydown={(e) => {
            if (e.key === 'Enter') {
              handleLoginWithGoogle(e)
            }
          }}
        />
        <Button variant="outline" onclick={handleLoginWithGoogle}>
          <Google />
          Google
        </Button>
      </Card.Content>
      <Card.Footer></Card.Footer>
    </Card.Root>
  {/if}
</div>
