<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import * as Resizable from '$lib/components/ui/resizable/index'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'
  import Button from '$lib/components/ui/button/button.svelte'
  import type { Envelope, Mailbox } from '$lib/types'
  import EnvelopeList from './EnvelopeList.svelte'
  import MailboxList from './MailboxList.svelte'

  let selectedMailbox: Mailbox | null = $state(null)
  let selectedMailUid: number = $state(0)

  let user: Promise<string> = invoke('get_user')
  let mailboxesRequest: Promise<Mailbox[]> = invoke<Mailbox[]>('get_mailboxes')

  let messageRequest: Promise<string> = $derived.by(() => {
    return invoke('get_mail_content', {
      mailbox: selectedMailbox?.name ?? 'INBOX',
      uid: selectedMailUid,
    })
  })
  let envelopesRequest: Promise<Envelope[]> = $derived.by(() => {
    return invoke<Envelope[]>('get_envelopes', {
      mailbox: selectedMailbox?.name ?? 'INBOX',
    }).then((envelopes: Envelope[]) =>
      envelopes.toSorted(
        (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime()
      )
    )
  })

  const handleSelectMailbox = (mailbox: Mailbox) => {
    selectedMailbox = mailbox
  }

  const handleSelectEnvelope = (envelope: Envelope) => {
    selectedMailUid = envelope.uid
  }
</script>

<main class="h-screen">
  <Resizable.PaneGroup direction="horizontal">
    <Resizable.Pane
      minSize={20}
      defaultSize={20}
      class="border-r border-gray-200"
    >
      {#await user}
        <span>Loading user...</span>
      {:then awaited}
        <h1 class="text-xl font-bold">{awaited}</h1>
      {:catch error}
        <span>Error: {error.message}</span>
      {/await}
      {#await mailboxesRequest}
        <div class="h-full flex items-center justify-center">
          <LoadingSpinner />
        </div>
      {:then mailboxes}
        <MailboxList items={mailboxes} onselect={handleSelectMailbox} />
      {:catch error}
        <span class="error-msg">Error: {error.message}</span>
      {/await}
    </Resizable.Pane>
    <Resizable.Handle />
    <Resizable.Pane minSize={20} defaultSize={30}>
      <h2 class="text-sm">{selectedMailbox?.name}</h2>
      {#await envelopesRequest}
        <div class="h-full flex items-center justify-center">
          <LoadingSpinner />
        </div>
      {:then envelopes}
        <EnvelopeList items={envelopes} onselect={handleSelectEnvelope} />
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
      <div class="mail h-dvh">
        {#await messageRequest}
          <div class="h-full flex items-center justify-center">
            <LoadingSpinner />
          </div>
        {:then awaited}
          <iframe
            class="w-full h-full"
            title="Email message"
            sandbox="allow-same-origin"
            srcdoc={awaited}
          ></iframe>
        {:catch error}
          <span class="error-msg">Error: {error.message}</span>
        {/await}
      </div>
    </Resizable.Pane>
  </Resizable.PaneGroup>
</main>

<style>
  .error-msg {
    color: red;
  }
</style>
