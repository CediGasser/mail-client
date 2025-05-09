import type { LayoutLoad } from './$types'
import { getEnvelopes } from '$lib/commands'

export const load: LayoutLoad = async ({ params, depends, parent }) => {
  depends('data:envelopes')

  const mailboxName = decodeURIComponent(params.mailbox)
  const { mailboxes } = await parent()
  const mailbox = mailboxes.find((m) => m.name === mailboxName)

  // Get the messages for the mailbox
  const envelopes = getEnvelopes(mailboxName).then((envelopes) => {
    envelopes.sort((a, b) => {
      return b.date.getTime() - a.date.getTime()
    })
    return envelopes
  })

  return {
    envelopes,
    mailbox,
  }
}
