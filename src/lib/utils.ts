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

export function formatMailbox(mailbox: Mailbox): Mailbox {
  mailbox.display_name.replace('[Gmail]/', '')

  if (mailbox.name === 'INBOX') {
    return { ...mailbox, display_name: 'Inbox' }
  }
  if (mailbox.attributes.includes('\\All')) {
    return { ...mailbox, display_name: 'All' }
  }
  if (mailbox.attributes.includes('\\Drafts')) {
    return { ...mailbox, display_name: 'Drafts' }
  }
  if (mailbox.attributes.includes('\\Sent')) {
    return { ...mailbox, display_name: 'Sent' }
  }
  if (mailbox.attributes.includes('\\Flagged')) {
    return { ...mailbox, display_name: 'Flagged' }
  }
  if (mailbox.attributes.includes('\\Trash')) {
    return { ...mailbox, display_name: 'Trash' }
  }
  if (mailbox.attributes.includes('\\Junk')) {
    return { ...mailbox, display_name: 'Junk' }
  }
  if (mailbox.attributes.includes('\\Important')) {
    return { ...mailbox, display_name: 'Important' }
  }
  return { ...mailbox }
}

export function getMailboxIconComponent(mailbox: Mailbox): Component {
  if (mailbox.name === 'INBOX') {
    return Mail
  }
  if (mailbox.attributes.includes('\\All')) {
    return Archive
  }
  if (mailbox.attributes.includes('\\Drafts')) {
    return PencilLine
  }
  if (mailbox.attributes.includes('\\Sent')) {
    return Send
  }
  if (mailbox.attributes.includes('\\Flagged')) {
    return Flag
  }
  if (mailbox.attributes.includes('\\Trash')) {
    return Trash
  }
  if (mailbox.attributes.includes('\\Junk')) {
    return ArchiveX
  }
  if (mailbox.attributes.includes('\\Important')) {
    return CircleAlert
  }
  return Folder
}

const DIVISIONS = [
  { amount: 60, name: 'seconds' },
  { amount: 60, name: 'minutes' },
  { amount: 24, name: 'hours' },
  { amount: 7, name: 'days' },
  { amount: 4.34524, name: 'weeks' },
  { amount: 12, name: 'months' },
  { amount: Number.POSITIVE_INFINITY, name: 'years' },
] as const

const formatter = new Intl.RelativeTimeFormat(undefined, {
  numeric: 'auto',
})

export function formatTimeAgo(date: Date) {
  let duration = (date.getTime() - new Date().getTime()) / 1000

  for (let i = 0; i <= DIVISIONS.length; i++) {
    const division = DIVISIONS[i]
    if (Math.abs(duration) < division.amount) {
      return formatter.format(Math.round(duration), division.name)
    }
    duration /= division.amount
  }
}
