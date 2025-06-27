import type { PageLoad } from './$types'
import { getAccount } from '$lib/mail/account.svelte'

export const load: PageLoad = async ({ params }) => {
  const mailbox = decodeURIComponent(params.mailbox)
  const uid = params.uid && parseInt(params.uid, 10)
  const email = params.account

  const account = getAccount(email)

  if (!uid) {
    return {
      uid: null,
    }
  }

  account.getMailbox(mailbox)?.getMessage(uid)?.loadMessageBody()

  return {
    uid,
  }
}
