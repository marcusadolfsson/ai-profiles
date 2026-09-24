import type { RemoteAccount, RemoteHost } from '@/lib/types'

import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { ToastProvider } from '@/design'
import { remoteLoginStart, remoteLoginSubmit, remoteLogout } from '@/lib/commands'
import { renderWithQuery } from '@/test/render-with-query'

import { SwitchAccountDialog } from './switch-account-dialog'

vi.mock('@/lib/commands', async () => {
  const actual = await vi.importActual<typeof import('@/lib/commands')>('@/lib/commands')
  return {
    ...actual,
    remoteLogout: vi.fn(),
    remoteLoginStart: vi.fn(),
    remoteLoginSubmit: vi.fn(),
    remoteLoginCancel: vi.fn(async () => {}),
    openExternalUrl: vi.fn(async () => {}),
  }
})

const host: RemoteHost = {
  id: 'h1',
  label: 'xjopa1',
  hostname: 'xjopa1',
  addresses: ['100.110.48.97:7443'],
  fingerprint: '09:BB',
  clientId: 'c1',
  pairedAt: '2026-09-22T00:00:00Z',
  lastGoodAddress: null,
  profiles: {},
}

function signedIn(email: string, extra: Partial<RemoteAccount> = {}): RemoteAccount {
  return {
    name: 'brain',
    isDefault: false,
    configDir: '/home/marcus/.claude-accounts/brain',
    account: { email, name: 'Marcus', organization: null, plan: 'Max' },
    signedIn: true,
    signedInUntil: null,
    sessions: 5,
    runningSessions: 2,
    ...extra,
  }
}

function renderDialog(onClose = vi.fn()) {
  renderWithQuery(
    <ToastProvider>
      <SwitchAccountDialog open host={host} account={signedIn('marcus1@example.com')} onClose={onClose} />
    </ToastProvider>,
  )
  return onClose
}

async function switchAndSignIn() {
  const user = userEvent.setup()
  expect(screen.getByText(/Its 2 sessions stop, and resume under the new account/)).toBeInTheDocument()
  await user.click(screen.getByRole('button', { name: /Switch account/ }))
  expect(remoteLogout).toHaveBeenCalledWith({
    hostId: 'h1',
    account: 'brain',
    stopRunning: true,
    resumeAfterSignIn: true,
  })
  await user.type(await screen.findByLabelText('Code'), 'abc#123')
  await user.click(screen.getByRole('button', { name: /^Sign in/ }))
}

beforeEach(() => {
  vi.mocked(remoteLogout).mockReset()
  vi.mocked(remoteLogout).mockResolvedValue(2)
  vi.mocked(remoteLoginStart).mockResolvedValue({
    loginId: 'l1',
    url: 'https://claude.com/cai/oauth/authorize?x=1',
    expiresAt: '2026-09-24T12:00:00Z',
  })
})

describe('SwitchAccountDialog', () => {
  it('signs out keeping the sessions to resume, signs in as the other account, and says so', async () => {
    vi.mocked(remoteLoginSubmit).mockResolvedValue(signedIn('marcus2@example.com', { pendingResume: 2 }))
    const onClose = renderDialog()
    await switchAndSignIn()
    expect(await screen.findByText('brain is on marcus2@example.com')).toBeInTheDocument()
    expect(screen.getByText('2 sessions resuming, in the same conversations.')).toBeInTheDocument()
    expect(onClose).toHaveBeenCalled()
  })

  it('notices when the browser signed it in to the same account again', async () => {
    vi.mocked(remoteLoginSubmit).mockResolvedValue(signedIn('Marcus1@example.com'))
    const onClose = renderDialog()
    await switchAndSignIn()
    expect(await screen.findByText('Still marcus1@example.com')).toBeInTheDocument()
    expect(onClose).not.toHaveBeenCalled()
    await userEvent.setup().click(screen.getByRole('button', { name: 'Sign in as another' }))
    expect(remoteLogout).toHaveBeenCalledTimes(2)
  })
})
