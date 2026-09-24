import type { RemoteAccount, RemoteHost } from '@/lib/types'

import { useEffect, useState } from 'react'

import { Button, Dialog, Kbd, useToast } from '@/design'
import { sessionErrorMessage } from '@/features/profiles/components/session-error-message'

import { useSwitchAccountSignOut } from '../api/use-remote'
import { SignInDialog } from './sign-in-dialog'

type Props = {
  open: boolean
  host: RemoteHost
  account: RemoteAccount
  onClose: () => void
}

type Step = 'confirm' | 'signingIn' | 'sameAccount'

function sessions(count: number): string {
  return `${count} ${count === 1 ? 'session' : 'sessions'}`
}

/**
 * Switching a profile to another Claude account, the way to go on when its
 * account runs out: its running sessions stop, it signs out, and it signs in
 * as the other account in the browser. The host then resumes the same
 * sessions under it. Nothing moves, and no other profile is touched.
 */
export function SwitchAccountDialog({ open, host, account, onClose }: Props) {
  const signOut = useSwitchAccountSignOut(host.id, account.name)
  const toast = useToast()
  const [step, setStep] = useState<Step>('confirm')
  const [previous, setPrevious] = useState<string | null>(null)
  const running = account.runningSessions
  const current = account.account?.email ?? null

  // biome-ignore lint/correctness/useExhaustiveDependencies: start over each time it opens
  useEffect(() => {
    if (open) {
      setStep('confirm')
      signOut.reset()
    }
  }, [open])

  async function handleSwitch() {
    setPrevious(current ?? previous)
    try {
      await signOut.mutateAsync()
      setStep('signingIn')
    } catch {
      // Shown below.
    }
  }

  function handleSignedIn(signed: RemoteAccount) {
    const now = signed.account?.email ?? null
    if (previous !== null && now !== null && previous.toLowerCase() === now.toLowerCase()) {
      setStep('sameAccount')
      return
    }
    const resuming = signed.pendingResume ?? 0
    toast.success(
      `${account.name} is on ${now ?? 'another account'}`,
      resuming > 0 ? `${sessions(resuming)} resuming, in the same conversations.` : `On ${host.label}.`,
    )
    onClose()
  }

  if (!open) {
    return null
  }

  if (step === 'signingIn') {
    return (
      <SignInDialog
        open
        host={host}
        account={account.name}
        title={`Sign ${account.name} in to the other account`}
        cancelLabel="Later"
        onSignedIn={handleSignedIn}
        onClose={onClose}
      />
    )
  }

  if (step === 'sameAccount') {
    return (
      <Dialog
        open
        title={`Still ${previous}`}
        description={`${account.name} signed in to the same account again.`}
        onClose={onClose}
        foot={
          <>
            <Button variant="ghost" size="sm" trailingKbd={<Kbd>⎋</Kbd>} onClick={onClose}>
              Keep it
            </Button>
            <Button variant="primary" size="sm" disabled={signOut.isPending} onClick={handleSwitch}>
              {signOut.isPending ? 'Signing out…' : 'Sign in as another'}
            </Button>
          </>
        }
      >
        <p className="text-body text-ink-soft">
          The browser was still signed in to claude.ai as {previous}. Switch account on claude.ai first, then sign in
          again. Its sessions are running again meanwhile.
        </p>
      </Dialog>
    )
  }

  return (
    <Dialog
      open
      title={`Switch ${account.name} to another account?`}
      description={current ? `Signed in as ${current}, on ${host.label}.` : `On ${host.label}.`}
      onClose={onClose}
      onSubmit={handleSwitch}
      foot={
        <>
          <Button variant="ghost" size="sm" trailingKbd={<Kbd>⎋</Kbd>} disabled={signOut.isPending} onClick={onClose}>
            Cancel
          </Button>
          <Button
            variant="primary"
            size="sm"
            trailingKbd={<Kbd variant="onOrange">⏎</Kbd>}
            disabled={signOut.isPending}
            onClick={handleSwitch}
          >
            {signOut.isPending ? 'Signing out…' : 'Switch account'}
          </Button>
        </>
      }
    >
      <div className="space-y-2.5 text-body text-ink-soft">
        <p>
          {running > 0
            ? `Its ${sessions(running)} ${running === 1 ? 'stops' : 'stop'}, and ${running === 1 ? 'resumes' : 'resume'} under the new account as soon as it's signed in: the same conversations, with Remote Control on. `
            : ''}
          Nothing moves, and no other profile is touched.
        </p>
        <p className="text-muted">
          A sign-in page opens in your browser. Sign in there as the other account
          {current ? `: if claude.ai shows ${current}, switch account on claude.ai first` : ''}.
        </p>
        {signOut.isError ? (
          <p role="alert" className="text-meta text-red">
            {sessionErrorMessage(signOut.error, 'It could not be signed out.')}
          </p>
        ) : null}
      </div>
    </Dialog>
  )
}
