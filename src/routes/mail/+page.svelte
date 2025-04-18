<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import * as Resizable from '$lib/components/ui/resizable/index'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'
  import Button from '$lib/components/ui/button/button.svelte'
  import type { Envelope } from '$lib/types'
  import EnvelopeList from './EnvelopeList.svelte'

  type GmailOauth = {
    user: string
    access_token: string
  }

  let gmailOauth: Promise<GmailOauth> | null = $state(null)
  let firstMessage: Promise<string> | null = $state(null)
  let envelopesRequest: Promise<Envelope[]> | null = $state(null)

  gmailOauth = invoke('get_gmail_oauth')
  firstMessage = invoke('get_mail_content')
  envelopesRequest = invoke<Envelope[]>('get_envelopes').then(
    (envelopes: Envelope[]) =>
      envelopes.toSorted(
        (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime()
      )
  )

  const handleSelectEnvelope = (envelope: Envelope) => {
    firstMessage = invoke('get_mail_content', { uid: envelope.uid })
  }
</script>

<main class="h-screen">
  <Resizable.PaneGroup direction="horizontal">
    <Resizable.Pane minSize={20} defaultSize={30}>
      {#if gmailOauth !== null}
        {#await gmailOauth}
          <span>Loading user...</span>
        {:then awaited}
          <div class="gmail-oauth">
            <p>User: {awaited.user}</p>
          </div>
        {:catch error}
          <div class="error">
            <p>Error: {error.message}</p>
          </div>
        {/await}
      {/if}
      {#if envelopesRequest !== null}
        {#await envelopesRequest}
          <div class="h-full flex items-center justify-center">
            <LoadingSpinner />
          </div>
        {:then envelopes}
          <EnvelopeList items={envelopes} onselect={handleSelectEnvelope} />
        {:catch error}
          <li class="error-msg">Error: {error.message}</li>
        {/await}
      {/if}
    </Resizable.Pane>
    <Resizable.Handle />
    <Resizable.Pane minSize={20}>
      <div class="flex flex-row items-center justify-center gap-2">
        <h1 class="text-2xl align-middle">Mail</h1>
        <Button variant="outline" class="mb-4" href="/mail/new">Compose</Button>
      </div>
      <div class="mail h-dvh">
        {#await firstMessage}
          <div class="h-full flex items-center justify-center">
            <LoadingSpinner />
          </div>
        {:then awaited}
          <span>{@html awaited}</span>
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
