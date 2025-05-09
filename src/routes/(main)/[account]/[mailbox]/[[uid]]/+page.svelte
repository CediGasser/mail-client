<script lang="ts">
  import * as Resizable from '$lib/components/ui/resizable/index'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'
  import Button from '$lib/components/ui/button/button.svelte'
  import EnvelopeList from './EnvelopeList.svelte'
  import Separator from '$lib/components/ui/separator/separator.svelte'
  import Star from '@lucide/svelte/icons/star'
  import MessageComponent from './Message.svelte'

  let { data } = $props()

  let messageLoaded = $state(false)

  data.message?.then(() => {
    messageLoaded = true
  })
</script>

<Resizable.PaneGroup direction="horizontal">
  <Resizable.Pane minSize={20} defaultSize={30}>
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
      <EnvelopeList items={envelopes} />
    {:catch error}
      <li class="error-msg">Error: {error.message}</li>
    {/await}
  </Resizable.Pane>
  <Resizable.Handle />
  <Resizable.Pane minSize={20}>
    <header class="flex flex-row justify-between items-center p-2">
      <Button disabled={!messageLoaded} variant="outline" href="/mail/new">
        <Star />
      </Button>
    </header>
    <Separator />
    {#if data.message}
      {#await data.message}
        <div class="h-full flex items-center justify-center">
          <LoadingSpinner />
        </div>
      {:then message}
        <MessageComponent {message} />
      {:catch error}
        <li class="error-msg">Error: {error.message}</li>
      {/await}
    {/if}
  </Resizable.Pane>
</Resizable.PaneGroup>
