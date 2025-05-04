import type { PageLoad } from './$types'
import { getMailContent } from '$lib/commands'

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
  const message = getMailContent(mailbox, parseInt(uid))

  return {
    message,
    uid,
  }
}
