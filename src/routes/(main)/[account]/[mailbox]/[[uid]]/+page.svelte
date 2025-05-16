<script lang="ts">
  import * as Resizable from '$lib/components/ui/resizable/index'
  import * as Sidebar from '$lib/components/ui/sidebar/index'
  import LoadingSpinner from '$lib/components/custom/LoadingSpinner.svelte'
  import Button from '$lib/components/ui/button/button.svelte'
  import EnvelopeList from './EnvelopeList.svelte'
  import Separator from '$lib/components/ui/separator/separator.svelte'
  import Star from '@lucide/svelte/icons/star'
  import MessageComponent from './Message.svelte'
  import {
    addFlags,
    archiveMessage,
    deleteMessage,
    removeFlags,
  } from '$lib/commands'
  import { invalidate } from '$app/navigation'
  import { Input } from '$lib/components/ui/input'
  import Trash from '@lucide/svelte/icons/trash'
  import { navigateTo } from '$lib/navigation'
  import Archive from '@lucide/svelte/icons/archive'

  let { data } = $props()

  let search = $state('')

  const handleToggleFlagged = async (e: MouseEvent) => {
    e.preventDefault()
    if (!data.message) return
    const message = await data.message
    try {
      if (message?.starred) {
        await removeFlags(
          data.account,
          message.mailbox_name,
          message.uid,
          '\\Flagged'
        )
      } else {
        await addFlags(
          data.account,
          message.mailbox_name,
          message.uid,
          '\\Flagged'
        )
      }
      invalidate('data:message')
    } catch (error) {
      console.error('Error toggling star:', error)
    }
  }

  const handleDelete = async (e: MouseEvent) => {
    e.preventDefault()
    if (!data.message) return
    const message = await data.message
    try {
      await deleteMessage(data.account, message.mailbox_name, message.uid)

      await navigateTo(data.account, message.mailbox_name, null)

      invalidate('data:envelopes')
      invalidate('data:message')
    } catch (error) {
      console.error('Error deleting message:', error)
    }
  }

  const handleArchive = async (e: MouseEvent) => {
    e.preventDefault()
    if (!data.message) return
    const message = await data.message
    try {
      await archiveMessage(data.account, message.mailbox_name, message.uid)

      await navigateTo(data.account, message.mailbox_name, null)

      invalidate('data:envelopes')
      invalidate('data:message')
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
          {data.mailbox?.display_name ?? 'Mailbox'}
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
    {#await data.envelopes}
      <div class="h-full flex items-center justify-center">
        <LoadingSpinner />
      </div>
    {:then envelopes}
      <EnvelopeList {search} account={data.account} items={envelopes} />
    {:catch error}
      <li class="error-msg">Error: {error.message}</li>
    {/await}
  </Resizable.Pane>
  {#if data.message}
    <Resizable.Handle />
  {/if}
  {#if data.message}
    <Resizable.Pane minSize={20}>
      {#await data.message}
        <div class="h-full flex items-center justify-center">
          <LoadingSpinner />
        </div>
      {:then message}
        <header class="flex flex-row gap-3 p-2">
          <Button onclick={handleToggleFlagged} variant="outline">
            <Star fill={message.starred ? 'black' : 'transparent'} />
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
      {:catch error}
        <li class="error-msg">Error: {error.message}</li>
      {/await}
    </Resizable.Pane>
  {/if}
</Resizable.PaneGroup>
