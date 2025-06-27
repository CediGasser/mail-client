<script lang="ts">
  import { Button } from '$lib/components/ui/button'
  import * as Sidebar from '$lib/components/ui/sidebar/index.js'
  import { getLinkTo } from '$lib/navigation'
  import type { Mailbox } from '$lib/mail/mailbox.svelte'
  import { getMailboxIconComponent } from '$lib/utils'
  import type { Component } from 'svelte'
  import PencilLine from '@lucide/svelte/icons/pencil-line'

  type MailboxWithIcon = Mailbox & {
    icon: Component
  }

  interface Props {
    email: string
    mailboxes: Mailbox[]
  }
  let { email, mailboxes }: Props = $props()

  const mailboxesWithIcons = mailboxes.filter(
    (mailbox) => !mailbox.attributes.includes('NoSelect')
  )
</script>

<Sidebar.Root variant="inset" collapsible="icon">
  <Sidebar.Header></Sidebar.Header>
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupLabel>Mailboxes</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each mailboxesWithIcons as mailbox}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                {#snippet child({ props })}
                  <a href={getLinkTo(email, mailbox.name, null)} {...props}>
                    <mailbox.icon></mailbox.icon>
                    <span>{mailbox.displayName}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Footer>
    <Sidebar.MenuButton>
      {#snippet child({ props })}
        <Button {...props} variant="default" href="/mail/new">
          <PencilLine />
          <span>Compose</span>
        </Button>
      {/snippet}
    </Sidebar.MenuButton>
  </Sidebar.Footer>
</Sidebar.Root>
