<script lang="ts">
  import * as Resizable from '$lib/components/ui/resizable/index'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'
  import Button from '$lib/components/ui/button/button.svelte'
  import type { Envelope } from '$lib/types'
  import EnvelopeList from './EnvelopeList.svelte'
  import { navigateTo } from '$lib/navigation'

  let { data } = $props()

  const handleSelectEnvelope = (envelope: Envelope) => {
    navigateTo(null, null, envelope.uid)
  }
</script>

<Resizable.PaneGroup direction="horizontal">
  <Resizable.Pane minSize={20} defaultSize={30}>
    {#await data.envelopes}
      <div class="h-full flex items-center justify-center">
        <LoadingSpinner />
      </div>
    {:then envelopes}
      <EnvelopeList mailbox={data.mailbox} items={envelopes} />
    {:catch error}
      <li class="error-msg">Error: {error.message}</li>
    {/await}
  </Resizable.Pane>
  <Resizable.Handle />
  <Resizable.Pane minSize={20}>
    <div class="flex flex-row items-center justify-center gap-2">
      <h1 class="text-2xl align-middle">Mail</h1>
      <Button variant="outline" class="mb-4" href="/mail/new">Compose</Button>
    </div>
    <div class="mail h-full">
      {#await data.message}
        <div class="h-full flex items-center justify-center">
          <LoadingSpinner />
        </div>
      {:then message}
        <iframe
          class="w-full h-full"
          title="Email message"
          sandbox="allow-same-origin"
          srcdoc={message}
        ></iframe>
      {:catch error}
        <span class="error-msg">Error: {error.message}</span>
      {/await}
    </div>
  </Resizable.Pane>
</Resizable.PaneGroup>
