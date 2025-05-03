<script lang="ts">
  import * as Resizable from '$lib/components/ui/resizable/index'
  import type { Mailbox } from '$lib/types'
  import MailboxList from './MailboxList.svelte'
  import { navigateTo } from '$lib/navigation'
  let { data, children } = $props()

  let { mailboxes } = data

  const handleSelectMailbox = (mailbox: Mailbox) => {
    navigateTo(null, mailbox.name, null)
  }
</script>

<main class="h-screen">
  <Resizable.PaneGroup direction="horizontal">
    <Resizable.Pane
      minSize={20}
      defaultSize={20}
      class="border-r border-gray-200"
    >
      <MailboxList items={mailboxes} onselect={handleSelectMailbox} />
    </Resizable.Pane>
    <Resizable.Handle />
    <Resizable.Pane minSize={50}>
      {@render children()}
    </Resizable.Pane>
  </Resizable.PaneGroup>
</main>
