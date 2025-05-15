import { invoke } from '@tauri-apps/api/core'
import type { AccountConfig, Envelope, Mailbox, Message } from '$lib/types'

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
  let envelopes = await invoke<Envelope[]>('get_envelopes', { email, mailbox })

  envelopes.forEach((envelope) => {
    envelope.date = new Date(envelope.date)
  })

  return envelopes
}

export async function getMessage(
  email: string,
  mailbox: string,
  uid: number
): Promise<Message> {
  return invoke<Message>('get_message', { email, mailbox, uid })
}

export async function sendEmail(
  email: string,
  to: string,
  subject: string,
  body: string
): Promise<string> {
  return invoke('send_email', { email, to, subject, body })
}

type Flag = '\\Seen' | '\\Answered' | '\\Flagged' | '\\Deleted' | '\\Draft'

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
