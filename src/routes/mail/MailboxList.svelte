<script lang="ts">
  import type { Mailbox } from '$lib/types'
  import MailboxComponent from './Mailbox.svelte'

  interface Props {
    items: Mailbox[]
    onselect?: (mailbox: Mailbox) => void
  }

  let { items, onselect }: Props = $props()

  let selectedMailbox: string = $state('INBOX')
  let filteredMailboxes: Mailbox[] = $derived(
    items.filter((mailbox) => !mailbox.attributes.includes('NoSelect'))
  )

  const handleSelectMailbox = (mailbox: Mailbox) => {
    selectedMailbox = mailbox.name
    onselect?.(mailbox)
  }
</script>

<main class="h-full">
  <div class="flex flex-col">
    {#each filteredMailboxes as mailbox}
      <MailboxComponent
        {...mailbox}
        onselect={handleSelectMailbox}
        selected={selectedMailbox === mailbox.name}
      />
    {/each}
  </div>
</main>
