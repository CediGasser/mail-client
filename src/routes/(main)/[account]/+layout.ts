import type { LayoutLoad } from './$types'
import { getMailboxes } from '$lib/commands'
import { formatMailbox } from '$lib/utils'

export const load: LayoutLoad = async ({ params }) => {
  const { account } = params

  const mailboxes = (await getMailboxes(account)).map(formatMailbox)

  return {
    mailboxes,
    account,
  }
}
