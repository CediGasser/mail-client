import type { PageLoad } from './$types'
import { getMessage } from '$lib/commands'

export const load: PageLoad = async ({ params }) => {
  const mailbox = decodeURIComponent(params.mailbox)
  const uid = params.uid

  if (!uid) {
    return {
      message: null,
      uid: null,
    }
  }

  // Get the message for the uid
  const message = getMessage(mailbox, parseInt(uid, 10))

  return {
    message,
    uid,
  }
}
