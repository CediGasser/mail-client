import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'
import type { Mailbox } from './types'
import type { Component } from 'svelte'
import Mail from '@lucide/svelte/icons/mail'
import Archive from '@lucide/svelte/icons/archive'
import PencilLine from '@lucide/svelte/icons/pencil-line'
import Send from '@lucide/svelte/icons/send'
import Flag from '@lucide/svelte/icons/flag'
import Trash from '@lucide/svelte/icons/trash'
import ArchiveX from '@lucide/svelte/icons/archive-x'
import CircleAlert from '@lucide/svelte/icons/circle-alert'
import Folder from '@lucide/svelte/icons/folder'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function getDomainFromMail(mail: string) {
  return mail.split('@')[1]
}

export async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export function formatMailbox(mailbox: Mailbox): Mailbox & { icon: Component } {
  mailbox.display_name.replace('[Gmail]/', '')

  if (mailbox.name === 'INBOX') {
    return { ...mailbox, display_name: 'Inbox', icon: Mail }
  }
  if (mailbox.attributes.includes('\\All')) {
    return { ...mailbox, display_name: 'All', icon: Archive }
  }
  if (mailbox.attributes.includes('\\Drafts')) {
    return { ...mailbox, display_name: 'Drafts', icon: PencilLine }
  }
  if (mailbox.attributes.includes('\\Sent')) {
    return { ...mailbox, display_name: 'Sent', icon: Send }
  }
  if (mailbox.attributes.includes('\\Flagged')) {
    return { ...mailbox, display_name: 'Flagged', icon: Flag }
  }
  if (mailbox.attributes.includes('\\Trash')) {
    return { ...mailbox, display_name: 'Trash', icon: Trash }
  }
  if (mailbox.attributes.includes('\\Junk')) {
    return { ...mailbox, display_name: 'Junk', icon: ArchiveX }
  }
  if (mailbox.attributes.includes('\\Important')) {
    return { ...mailbox, display_name: 'Important', icon: CircleAlert }
  }
  return { ...mailbox, icon: Folder }
}
