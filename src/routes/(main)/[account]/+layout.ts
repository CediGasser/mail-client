import type { LayoutLoad } from './$types'
import { getMailboxes } from '$lib/commands'

export const load: LayoutLoad = async ({ params }) => {
  const { account } = params

  const mailboxes = await getMailboxes()

  return {
    mailboxes,
    account,
  }
}
