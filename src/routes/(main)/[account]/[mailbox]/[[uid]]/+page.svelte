<script lang="ts">
  import * as Resizable from '$lib/components/ui/resizable/index'
  import * as Sidebar from '$lib/components/ui/sidebar/index'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'
  import Button from '$lib/components/ui/button/button.svelte'
  import EnvelopeList from './EnvelopeList.svelte'
  import Separator from '$lib/components/ui/separator/separator.svelte'
  import Star from '@lucide/svelte/icons/star'
  import MessageComponent from './Message.svelte'
  import { Input } from '$lib/components/ui/input'
  import Trash from '@lucide/svelte/icons/trash'
  import { navigateTo } from '$lib/navigation'
  import Archive from '@lucide/svelte/icons/archive'
  import { getAccount } from '$lib/mail/account.svelte'

  let { data } = $props()
  let account = getAccount(data.email)
  let mailbox = $derived(account.getMailbox(data.mailbox))
  let message = $derived(
    data.uid !== null ? mailbox?.getMessage(data.uid) : undefined
  )
  let starred = $derived(message?.starred ?? false)

  let search = $state('')

  const handleToggleFlagged = async () => {
    if (!message) return
    await message.toggleStarred()
  }

  const handleDelete = async () => {
    if (!message) return
    try {
      await mailbox?.deleteMessage(message.uid)

      await navigateTo(account.email, message.mailbox.name, null)
    } catch (error) {
      console.error('Error deleting message:', error)
    }
  }

  const handleArchive = async () => {
    if (!message) return
    try {
      await mailbox?.archiveMessage(message.uid)

      await navigateTo(account.email, message.mailbox.name, null)
    } catch (error) {
      console.error('Error archiving message:', error)
    }
  }
</script>

<Resizable.PaneGroup direction="horizontal">
  <Resizable.Pane minSize={20} defaultSize={30}>
    <header class="flex flex-row gap-3 items-center p-2">
      <div class="flex flex-row gap-3 items-center">
        <Sidebar.Trigger />
        <h2 class="text-2xl align-middle">
          {mailbox?.displayName ?? 'Mailbox'}
        </h2>
      </div>
      <Input
        type="text"
        placeholder="Search"
        class="max-w-xs"
        bind:value={search}
      />
    </header>
    <Separator />
    {#if mailbox?.syncState === 'syncing'}
      <div class="h-full flex items-center justify-center">
        <LoadingSpinner />
      </div>
    {:else if mailbox?.messages}
      <EnvelopeList {search} items={mailbox?.messages} />
    {/if}
  </Resizable.Pane>
  {#if message}
    <Resizable.Handle />
    <Resizable.Pane minSize={20}>
      {#if message.syncState === 'syncing'}
        <div class="h-full flex items-center justify-center">
          <LoadingSpinner />
        </div>
      {:else if message.syncState === 'idle'}
        <header class="flex flex-row gap-3 p-2">
          <Button onclick={handleToggleFlagged} variant="outline">
            <Star fill={starred ? 'black' : 'transparent'} />
          </Button>
          <Button onclick={handleArchive} variant="outline">
            <Archive />
          </Button>
          <Button onclick={handleDelete} variant="outline">
            <Trash />
          </Button>
        </header>
        <Separator />
        <MessageComponent {message} />
      {:else if message.syncState === 'error'}
        <li class="error-msg">Error</li>
      {/if}
    </Resizable.Pane>
  {/if}
</Resizable.PaneGroup>
