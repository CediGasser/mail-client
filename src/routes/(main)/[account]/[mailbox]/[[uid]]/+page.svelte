<script lang="ts">
  import * as Resizable from '$lib/components/ui/resizable/index'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'
  import Button from '$lib/components/ui/button/button.svelte'
  import EnvelopeList from './EnvelopeList.svelte'
  import Separator from '$lib/components/ui/separator/separator.svelte'
  import Star from '@lucide/svelte/icons/star'
  import MessageComponent from './Message.svelte'
  import { addFlags, removeFlags } from '$lib/commands'
  import { invalidate } from '$app/navigation'

  let { data } = $props()

  const handleToggleFlagged = async (e: MouseEvent) => {
    e.preventDefault()
    if (!data.message) return
    try {
      if (data.message?.starred) {
        await removeFlags(
          data.account,
          data.message.mailbox_name,
          data.message.uid,
          '\\Flagged'
        )
      } else {
        await addFlags(
          data.account,
          data.message.mailbox_name,
          data.message.uid,
          '\\Flagged'
        )
      }
      invalidate('data:envelopes')
    } catch (error) {
      console.error('Error toggling star:', error)
    }
  }
</script>

<Resizable.PaneGroup direction="horizontal">
  <Resizable.Pane minSize={20} defaultSize={data.message ? 30 : 100}>
    <header class="flex flex-row justify-between items-center p-2">
      <h2 class="text-2xl align-middle">
        {data.mailbox?.display_name ?? 'Mailbox'}
      </h2>
      <Button variant="outline" href="/mail/new">Compose</Button>
    </header>
    <Separator />
    {#await data.envelopes}
      <div class="h-full flex items-center justify-center">
        <LoadingSpinner />
      </div>
    {:then envelopes}
      <EnvelopeList account={data.account} items={envelopes} />
    {:catch error}
      <li class="error-msg">Error: {error.message}</li>
    {/await}
  </Resizable.Pane>
  {#if data.message}
    <Resizable.Handle />
  {/if}
  {#if data.message}
    <Resizable.Pane minSize={20}>
      <header class="flex flex-row justify-between items-center p-2">
        <button
          class="p-0 m-0 rounded-sm border-none bg-transparent {data.message
            .starred
            ? ''
            : 'text-gray-500 hover:text-black'} focus:outline-none"
          onclick={handleToggleFlagged}
        >
          <Star
            size="1rem"
            fill={data.message?.starred ? 'black' : 'transparent'}
          />
        </button>
      </header>
      <Separator />
      <MessageComponent message={data.message} />
    </Resizable.Pane>
  {/if}
</Resizable.PaneGroup>
