export type Mailbox = {
  name: string
  display_name: string
  delimiter: string
  attributes: string[]
}

export interface Envelope {
  uid: number
  from: string
  subject: string
  date: Date
  read?: boolean
  starred?: boolean
  mailbox_name: string
}

export interface Message extends Envelope {
  body: string
}

export type Address = {
  name: string
  email: string
}

export type Account = {
  email: string
}

export type AccountConfig = Account[]
