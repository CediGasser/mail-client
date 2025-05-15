<script lang="ts">
  import { Button } from '$lib/components/ui/button'
  import * as Sidebar from '$lib/components/ui/sidebar/index.js'
  import { getLinkTo } from '$lib/navigation'
  import type { Mailbox } from '$lib/types'
  import { getMailboxIconComponent } from '$lib/utils'
  import type { Component } from 'svelte'

  type MailboxWithIcon = Mailbox & {
    icon: Component
  }

  interface Props {
    mailboxes: Mailbox[]
    account: string
  }
  let { mailboxes, account }: Props = $props()

  const mailboxesWithIcons = mailboxes
    .filter((mailbox) => !mailbox.attributes.includes('NoSelect'))
    .map((mailbox) => {
      return {
        ...mailbox,
        icon: getMailboxIconComponent(mailbox),
      }
    }) as MailboxWithIcon[]
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
                  <a href={getLinkTo(account, mailbox.name, null)} {...props}>
                    <mailbox.icon></mailbox.icon>
                    <span>{mailbox.display_name}</span>
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
    <Button variant="default" href="/mail/new">Compose</Button>
  </Sidebar.Footer>
</Sidebar.Root>
