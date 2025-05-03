import { goto } from '$app/navigation'
import { page } from '$app/state'

export async function navigateTo(
  account: string | null,
  mailbox: string | null,
  messageUid: number | null
) {
  let path = page.url.pathname
  if (path.startsWith('/')) {
    // Remove the leading slash
    path = path.slice(1)
  }

  const [currentAccount, currentMailbox, currentMessageUid] = path.split('/')

  // If the account, mailbox, or messageUid is not provided, use the current ones
  account = account || currentAccount
  mailbox = mailbox || currentMailbox
  let uid = messageUid?.toString() || (currentMessageUid as string)

  // If the mailbox has changed, reset the messageUid
  if (mailbox !== currentMailbox) {
    uid = ''
  }

  // If the account has changed, reset the mailbox and messageUid
  if (account !== currentAccount) {
    mailbox = 'INBOX'
    uid = ''
  }

  mailbox = encodeURIComponent(mailbox)

  await goto(`/${account}/${mailbox}/${uid}`)
}
