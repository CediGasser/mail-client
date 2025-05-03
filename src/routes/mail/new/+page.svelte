<script lang="ts">
  import Input from '$lib/components/ui/input/input.svelte'
  import Label from '$lib/components/ui/label/label.svelte'
  import Textarea from '$lib/components/ui/textarea/textarea.svelte'
  import { Button } from '$lib/components/ui/button'
  import { sendEmail } from '$lib/commands'

  let toEmail: string = $state('')
  let subject: string = $state('')
  let body: string = $state('')

  const handleSendEmail = async () => {
    try {
      await sendEmail(toEmail, subject, body)
      alert('Email sent successfully!')
    } catch (error) {
      console.error('Error sending email:', error)
      alert('Failed to send email.')
      return
    }

    // Reset the form fields after sending the email
    toEmail = ''
    subject = ''
    body = ''
  }
</script>

<Button href="/" class="mb-4">Back to Inbox</Button>

<div class="flex flex-col gap-4">
  <Label for="to">To:</Label>
  <Input
    bind:value={toEmail}
    id="to"
    type="text"
    placeholder="Recipient's email"
  />
</div>

<div class="flex flex-col gap-4">
  <Label for="subject">Subject:</Label>
  <Input
    bind:value={subject}
    id="subject"
    type="text"
    placeholder="Email subject"
  />
</div>

<div class="flex flex-col gap-4">
  <Label for="body">Body:</Label>
  <Textarea bind:value={body} id="body" placeholder="Type your message here" />
</div>

<Button variant="default" class="mt-4" onclick={handleSendEmail}>
  Send Email
</Button>
