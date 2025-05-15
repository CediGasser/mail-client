import type { PageLoad } from './$types'
import { getMessage } from '$lib/commands'

export const load: PageLoad = async ({ params, parent }) => {
  const mailbox = decodeURIComponent(params.mailbox)
  const uid = params.uid
  const account = params.account

  if (!uid) {
    return {
      message: null,
      uid: null,
    }
  }

  // Get the message for the uid
  const message = await getMessage(account, mailbox, parseInt(uid, 10))

  return {
    message,
    uid,
  }
}
