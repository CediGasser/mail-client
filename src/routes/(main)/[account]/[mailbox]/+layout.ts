import type { LayoutLoad } from './$types'
import { getAccount } from '$lib/mail/account.svelte'

export const load: LayoutLoad = async ({ params, parent }) => {
  const { email } = await parent()
  const mailboxName = decodeURIComponent(params.mailbox)

  const account = getAccount(email)
  const mailbox = account.getMailbox(mailboxName)

  if (mailbox?.syncState === 'initial') {
    // If the mailbox is in initial state, we need to sync it first
    mailbox.syncMessages()
  } else if (mailbox?.syncState === 'error') {
    // If the mailbox is in error state, we can try to sync it again
    mailbox.syncMessages()
  } else if (mailbox?.syncState === 'idle') {
    // If the mailbox is idle, we can proceed without syncing
    // This is useful for cases where the mailbox has already been synced
    console.info(`Mailbox ${mailboxName} is already synced or in idle state.`)
  }

  return {
    mailbox: mailboxName,
  }
}
