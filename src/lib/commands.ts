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
