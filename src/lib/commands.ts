/*
            commands::login_with_google,
            commands::get_mail_content,
            commands::get_envelopes,
            commands::send_email,
            commands::get_mailboxes,
            */

import { invoke } from '@tauri-apps/api/core'
import type { AccountConfig, Envelope, Mailbox } from '$lib/types'

export async function getConfig(): Promise<AccountConfig> {
  return invoke<AccountConfig>('get_config')
}

export async function configAddAccount(email: string): Promise<void> {
  return invoke('config_add_account', { email })
}

export async function configRemoveAccount(email: string): Promise<void> {
  return invoke('config_remove_account', { email })
}

export async function loginWithGoogle(email: string): Promise<void> {
  return invoke('login_with_google', { email })
}

export async function getMailboxes(): Promise<Mailbox[]> {
  return invoke<Mailbox[]>('get_mailboxes')
}

export async function getEnvelopes(mailbox: string): Promise<Envelope[]> {
  let envelopes = await invoke<Envelope[]>('get_envelopes', { mailbox })

  envelopes.forEach((envelope) => {
    envelope.date = new Date(envelope.date)
  })

  return envelopes
}

export async function getMailContent(
  mailbox: string,
  uid: number
): Promise<string> {
  return invoke<string>('get_mail_content', { mailbox, uid })
}

export async function sendEmail(
  to: string,
  subject: string,
  body: string
): Promise<string> {
  return invoke('send_email', { to, subject, body })
}

export async function markAsFlagged(
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('mark_flagged', { mailbox, uid })
}

export async function unmarkAsFlagged(
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('unmark_flagged', { mailbox, uid })
}

export async function markAsSeen(mailbox: string, uid: number): Promise<void> {
  return invoke('mark_seen', { mailbox, uid })
}

export async function unmarkAsSeen(
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('unmark_seen', { mailbox, uid })
}

export async function markAsDeleted(
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('mark_deleted', { mailbox, uid })
}

export async function unmarkAsDeleted(
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('unmark_deleted', { mailbox, uid })
}

export async function markAsDraft(mailbox: string, uid: number): Promise<void> {
  return invoke('mark_draft', { mailbox, uid })
}

export async function unmarkAsDraft(
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('unmark_draft', { mailbox, uid })
}

export async function markAsAnswered(
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('mark_answered', { mailbox, uid })
}

export async function unmarkAsAnswered(
  mailbox: string,
  uid: number
): Promise<void> {
  return invoke('unmark_answered', { mailbox, uid })
}
