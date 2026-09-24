import type { RemoteAccount, RemoteHost, RemoteSession } from '@/lib/types'

import { screen } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { ToastProvider } from '@/design'
import { profileAccount, remoteListAccounts, remoteListHosts, remoteListSessions } from '@/lib/commands'
import { renderWithQuery } from '@/test/render-with-query'

import { ProfileRemoteControl } from './profile-remote-control'

vi.mock('@/lib/commands', async () => {
  const actual = await vi.importActual<typeof import('@/lib/commands')>('@/lib/commands')
  return {
    ...actual,
    profileAccount: vi.fn(),
    remoteListHosts: vi.fn(),
    remoteListAccounts: vi.fn(),
    remoteListSessions: vi.fn(),
  }
})

function host(id: string, label: string): RemoteHost {
  return {
    id,
    label,
    hostname: label,
    addresses: [`${label}:7443`],
    fingerprint: '00',
    clientId: 'c',
    pairedAt: '2026-09-22T00:00:00Z',
    lastGoodAddress: null,
    profiles: {},
  }
}

function profile(name: string, email: string): RemoteAccount {
  return {
    name,
    isDefault: false,
    configDir: `/home/marcus/.claude-accounts/${name}`,
    account: { email, name: 'Marcus', organization: null, plan: 'Max' },
    signedIn: true,
    signedInUntil: null,
    sessions: 2,
    runningSessions: 1,
  }
}

function session(id: string, title: string, remoteControl: boolean, running = true): RemoteSession {
  return {
    id,
    cwd: `/home/marcus/${title}`,
    title,
    named: true,
    lastPrompt: null,
    updatedAt: '2026-09-24T10:00:00Z',
    sizeBytes: 1,
    running,
    window: running ? { session: 'ai', windowId: '@1', paneId: '%1' } : null,
    remoteControl,
    bridgeSessionId: remoteControl ? `session_${id}` : null,
    waiting: false,
    remoteControlConnecting: false,
    empty: false,
  }
}

beforeEach(() => {
  vi.clearAllMocks()
  vi.mocked(remoteListHosts).mockResolvedValue([host('h1', 'xjopa1'), host('h2', 'ONE')])
  vi.mocked(remoteListAccounts).mockImplementation(async (hostId: string) =>
    hostId === 'h1'
      ? [profile('brain', 'marcus2@example.com'), profile('foawa', 'marcus1@example.com')]
      : [profile('crm', 'MARCUS2@example.com')],
  )
  vi.mocked(remoteListSessions).mockImplementation(async ({ hostId, account }) => {
    if (hostId === 'h1' && account === 'brain') {
      return [session('a1', 'Brain', true), session('a2', 'Old', false, false)]
    }
    if (hostId === 'h2' && account === 'crm') {
      return [session('b1', 'CRM', true), session('b2', 'No RC', false)]
    }
    return [session('c1', 'FOAWA', true)]
  })
})

function renderIt() {
  renderWithQuery(
    <ToastProvider>
      <ProfileRemoteControl profileId="p2" />
    </ToastProvider>,
  )
}

describe('ProfileRemoteControl', () => {
  it("lists the Remote Control sessions on every host whose profile is on this profile's account", async () => {
    vi.mocked(profileAccount).mockResolvedValue({
      status: 'signedIn',
      account: { email: 'marcus2@example.com', name: 'Marcus', organization: null, plan: 'Max' },
    })
    renderIt()
    expect(await screen.findByText('Remote Control on marcus2@example.com')).toBeInTheDocument()
    expect(await screen.findByText('Brain')).toBeInTheDocument()
    expect(await screen.findByText('CRM')).toBeInTheDocument()
    expect(screen.getByText('xjopa1/brain')).toBeInTheDocument()
    expect(screen.getByText('ONE/crm')).toBeInTheDocument()
    // Another account's session, one without Remote Control, and one that isn't running aren't listed.
    expect(screen.queryByText('FOAWA')).toBeNull()
    expect(screen.queryByText('No RC')).toBeNull()
    expect(screen.queryByText('Old')).toBeNull()
    expect(screen.getAllByRole('button', { name: 'Open in Claude' })).toHaveLength(2)
  })

  it('shows nothing for a profile that is signed out', async () => {
    vi.mocked(profileAccount).mockResolvedValue({ status: 'signedOut' })
    renderIt()
    await new Promise((resolve) => setTimeout(resolve, 50))
    expect(screen.queryByText(/Remote Control on/)).toBeNull()
    expect(remoteListHosts).not.toHaveBeenCalled()
  })
})
