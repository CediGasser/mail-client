<script lang="ts">
  import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte'
  import type { Mailbox } from '$lib/types'
  import MailboxComponent from './Mailbox.svelte'

  interface Props {
    items: Mailbox[]
    onselect?: (mailbox: Mailbox) => void
  }

  let { items, onselect }: Props = $props()

  let selectedMailbox: string = $state('INBOX')
  let filteredMailboxes: Mailbox[] = $derived(
    items
      .filter((mailbox) => !mailbox.attributes.includes('NoSelect'))
      .map(formatMailbox)
  )

  const handleSelectMailbox = (mailbox: Mailbox) => {
    selectedMailbox = mailbox.name
    onselect?.(mailbox)
  }

  function formatMailbox(mailbox: Mailbox): Mailbox {
    if (mailbox.name === 'INBOX') {
      return { ...mailbox, name: 'Inbox' }
    }
    return mailbox
  }
</script>

<ScrollArea class="h-full">
  <div class="flex flex-col">
    {#each filteredMailboxes as mailbox}
      <MailboxComponent
        {mailbox}
        onselect={handleSelectMailbox}
        selected={selectedMailbox === mailbox.name}
      />
    {/each}
  </div>
</ScrollArea>
