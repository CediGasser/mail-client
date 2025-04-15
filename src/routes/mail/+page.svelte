<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import * as Resizable from '$lib/components/ui/resizable/index'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'
  import MailEnvelope from '$lib/components/custom/MailEnvelope.svelte'

  type GmailOauth = {
    user: string
    access_token: string
  }

  type Envelope = {
    date: String
    subject: String
    from: String
    to: String
  }

  let gmailOauth: Promise<GmailOauth> | null = $state(null)
  let firstMessage: Promise<string> | null = $state(null)
  let envelopes: Promise<Envelope[]> | null = $state(null)

  gmailOauth = invoke('get_gmail_oauth')
  firstMessage = invoke('get_mail_content')
  envelopes = invoke('get_envelopes')
</script>

<Resizable.PaneGroup direction="horizontal">
  <Resizable.Pane minSize={20}>
    <h1>Inbox</h1>
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
    <ul>
      {#if envelopes !== null}
        {#await envelopes}
          <LoadingSpinner />
        {:then awaited}
          {#each awaited as envelope}
            <MailEnvelope
              date={envelope.date}
              subject={envelope.subject}
              from={envelope.from}
              to={envelope.to}
            />
          {/each}
        {:catch error}
          <li class="error-msg">Error: {error.message}</li>
        {/await}
      {/if}
    </ul>
  </Resizable.Pane>
  <Resizable.Handle />
  <Resizable.Pane minSize={20}>
    <h1>One Mail:</h1>
    <p>
      {#await firstMessage}
        <LoadingSpinner />
      {:then awaited}
        <span>{awaited}</span>
      {:catch error}
        <span class="error-msg">Error: {error.message}</span>
      {/await}
    </p>
  </Resizable.Pane>
</Resizable.PaneGroup>

<style>
  .error-msg {
    color: red;
  }
</style>
