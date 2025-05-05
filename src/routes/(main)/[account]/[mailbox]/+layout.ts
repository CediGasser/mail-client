import type { LayoutLoad } from './$types'
import { getEnvelopes } from '$lib/commands'

export const load: LayoutLoad = async ({ params, depends }) => {
  depends('data:envelopes')

  const mailbox = decodeURIComponent(params.mailbox)

  // Get the messages for the mailbox
  const envelopes = getEnvelopes(mailbox).then((envelopes) => {
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
