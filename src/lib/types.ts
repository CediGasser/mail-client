export type Mailbox = {
  name: string
  display_name: string
  delimiter: string
  attributes: string[]
}

export interface Envelope {
  uid: number
  date: Date
  from: EmailAddress[]
  to: EmailAddress[]
  cc: EmailAddress[]
  bcc: EmailAddress[]
  subject: string
  headers: Record<string, string>
  flags: Flag[]
  mailbox_name: string
}

export type Flag =
  | '\\Seen'
  | '\\Answered'
  | '\\Flagged'
  | '\\Deleted'
  | '\\Draft'
  | (string & {})

export interface Message extends Envelope {
  body: string
}

export type EmailAddress = {
  name: string | null
  address: string
}

export type Account = {
  email: string
}

export type AccountConfig = Account[]
