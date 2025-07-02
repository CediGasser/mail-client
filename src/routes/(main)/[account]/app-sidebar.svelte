<script lang="ts">
  import { Button } from '$lib/components/ui/button'
  import * as Sidebar from '$lib/components/ui/sidebar/index.js'
  import { getLinkTo, navigateTo } from '$lib/navigation'
  import PencilLine from '@lucide/svelte/icons/pencil-line'
  import type { Account } from '$lib/mail/account.svelte'
  import { MailboxAttribute } from '$lib/mail/mailbox.svelte'
  import { goto } from '$app/navigation'
  import { saveDraft } from '$lib/commands'
  import { Message } from '$lib/mail/message.svelte'

  interface Props {
    account: Account
  }
  let { account }: Props = $props()

  const filteredMailboxes = account.mailboxes.filter(
    (mailbox) => !mailbox.attributes.includes('NoSelect')
  )

  const handleCompose = async () => {
    const draftsMailbox = account.searchMailboxByAttribute(
      MailboxAttribute.DRAFTS
    )

    if (!draftsMailbox) {
      console.error('Drafts mailbox not found')
      return
    }

    try {
      const newMessage = new Message(
        draftsMailbox,
        undefined,
        new Date(),
        [],
        [],
        [],
        [],
        '',
        {},
        ['\\Draft'],
        ''
      )
      const newUid = await newMessage.save()
      draftsMailbox.messages.push(newMessage)

      await navigateTo(account.email, draftsMailbox.name, newUid ?? null)
    } catch (error) {
      console.error('Error creating draft:', error)
    }
  }
</script>

<Sidebar.Root variant="inset" collapsible="icon">
  <Sidebar.Header></Sidebar.Header>
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupLabel>Mailboxes</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each filteredMailboxes as mailbox}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                {#snippet child({ props })}
                  <a
                    href={getLinkTo(account.email, mailbox.name, null)}
                    {...props}
                  >
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
        <Button {...props} variant="default" onclick={handleCompose}>
          <PencilLine />
          <span>Compose</span>
        </Button>
      {/snippet}
    </Sidebar.MenuButton>
  </Sidebar.Footer>
</Sidebar.Root>
