import { invoke } from '@tauri-apps/api/core'
import type {
  Flag,
  AccountConfig,
  Envelope,
  Mailbox,
  Message,
  EmailAddress,
} from '$lib/types'

export async function getConfig(): Promise<AccountConfig> {
  return invoke<AccountConfig>('get_config')
}

export async function configAddAccount(email: string): Promise<void> {
  return invoke('config_add_account', { email })
}

export async function configRemoveAccount(email: string): Promise<void> {
  return invoke('config_remove_account', { email })
}

export async function loginWithGoogle(): Promise<void> {
  return invoke('login_with_google')
}

export async function getMailboxes(email: string): Promise<Mailbox[]> {
  return invoke<Mailbox[]>('get_mailboxes', { email })
}

export async function getEnvelopes(
  email: string,
  mailbox: string
): Promise<Envelope[]> {
  const envelopes = await invoke<Envelope[]>('get_envelopes', {
    email,
    mailbox,
  })

  console.log('Envelopes:', envelopes)

  return envelopes.map((envelope) => ({
    ...envelope,
    date: new Date(envelope.date),
  }))
}

export async function getMessage(
  email: string,
  mailbox: string,
  uid: number
): Promise<Message> {
  const message = await invoke<Message>('get_message', { email, mailbox, uid })

  // Ensure the date is a Date object
  if (typeof message.date === 'string') {
    message.date = new Date(message.date)
  }
  return message
}

export async function sendEmail(
  from: string,
  to: EmailAddress[],
  cc: EmailAddress[] = [],
  bcc: EmailAddress[] = [],
  subject: string,
  body: string
): Promise<string> {
  return invoke('send_email', { from, to, cc, bcc, subject, body })
}

export async function removeFlags(
  email: string,
  mailbox: string,
  uid: number,
  flags: Flag[] | Flag
) {
  if (typeof flags === 'string') {
    flags = [flags]
  }
  return invoke('remove_flags', { email, mailbox, uid, flags })
}

export async function addFlags(
  email: string,
  mailbox: string,
  uid: number,
  flags: Flag[] | Flag
) {
  if (typeof flags === 'string') {
    flags = [flags]
  }
  return invoke('add_flags', { email, mailbox, uid, flags })
}

export async function deleteMessage(
  email: string,
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('delete_message', { email, mailbox, uid })
}

export async function archiveMessage(
  email: string,
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('archive_message', { email, mailbox, uid })
}

export async function saveDraft(
  email: string,
  mailbox: string,
  uid: number | undefined,
  subject: string,
  body: string,
  to?: EmailAddress[],
  cc?: EmailAddress[],
  bcc?: EmailAddress[]
): Promise<number> {
  console.log('Saving draft:', {
    email,
    mailbox,
    uid,
    subject,
    body,
    to,
    cc,
    bcc,
  })

  const newUid = invoke<number>('save_draft', {
    email,
    mailbox,
    uid,
    subject,
    body,
    to,
    cc,
    bcc,
  })

  return newUid
}
