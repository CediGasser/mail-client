<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'

  type GmailOauth = {
    user: string
    access_token: string
  }

  let gmailOauth: Promise<GmailOauth> | null = $state(null)

  gmailOauth = invoke('get_gmail_oauth')
</script>

{#if gmailOauth !== null}
  {#await gmailOauth}
    <span>Loading...</span>
  {:then awaited}
    <div class="gmail-oauth">
      <h2>Gmail OAuth</h2>
      <p>User: {awaited.user}</p>
      <p>Access Token: {awaited.access_token}</p>
    </div>
  {:catch error}
    <div class="error">
      <p>Error: {error.message}</p>
    </div>
  {/await}
{/if}

<style>
  .gmail-oauth {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
