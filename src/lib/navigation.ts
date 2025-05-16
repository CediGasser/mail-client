import { goto } from '$app/navigation'
import { page } from '$app/state'

export function getLinkTo(
  account: string | null,
  mailbox: string | null,
  messageUid: number | null
): string {
  mailbox = encodeURIComponent(mailbox || '')

  let path = page.url.pathname
  if (path.startsWith('/')) {
    // Remove the leading slash
    path = path.slice(1)
  }
  const [currentAccount, currentMailbox] = path.split('/')

  // If the account, mailbox, or messageUid is not provided, use the current ones
  account = account || currentAccount
  mailbox = mailbox || currentMailbox
  let uid = messageUid?.toString() || ''

  // If the mailbox has changed, reset the messageUid
  if (mailbox !== currentMailbox) {
    uid = ''
  }

  // If the account has changed, reset the mailbox and messageUid
  if (account !== currentAccount) {
    mailbox = 'INBOX'
    uid = ''
  }

  // Construct the link
  const link = `/${account}/${mailbox}/${uid}`
  return link
}

export async function navigateTo(
  account: string | null,
  mailbox: string | null,
  messageUid: number | null
) {
  const link = await getLinkTo(account, mailbox, messageUid)
  // Use the goto function to navigate to the new link
  console.log('Navigating to:', link)
  await goto(link)
}
