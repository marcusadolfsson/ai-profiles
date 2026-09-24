import type { RemoteAccount, RemoteHost, RemoteSession } from '@/lib/types'

import { screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { ToastProvider } from '@/design'
import { remoteListAccounts, remoteListHosts, remoteListSessions } from '@/lib/commands'
import { renderWithQuery } from '@/test/render-with-query'

import { RemoteControlOverview, RemoteControlSidebarRow } from './remote-control-overview'

vi.mock('@/lib/commands', async () => {
  const actual = await vi.importActual<typeof import('@/lib/commands')>('@/lib/commands')
  return {
    ...actual,
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

describe('RemoteControlOverview', () => {
  it('lists every connected session on every host, by account, a click from its profile', async () => {
    const onSelectProfile = vi.fn()
    renderWithQuery(
      <ToastProvider>
        <RemoteControlOverview onSelectProfile={onSelectProfile} />
      </ToastProvider>,
    )
    const marcus2 = await screen.findByRole('region', { name: 'marcus2@example.com' })
    expect(await within(marcus2).findByText('Brain')).toBeInTheDocument()
    expect(await within(marcus2).findByText('CRM')).toBeInTheDocument()
    const marcus1 = await screen.findByRole('region', { name: 'marcus1@example.com' })
    expect(await within(marcus1).findByText('FOAWA')).toBeInTheDocument()
    // Only what has Remote Control connected.
    expect(screen.queryByText('No RC')).toBeNull()
    expect(screen.queryByText('Old')).toBeNull()
    expect(screen.getAllByRole('button', { name: 'Open in Claude' })).toHaveLength(3)

    await userEvent.setup().click(within(marcus2).getByRole('button', { name: 'ONE/crm' }))
    expect(onSelectProfile).toHaveBeenCalledWith('remote:h2:crm')
  })

  it('counts them in the sidebar', async () => {
    renderWithQuery(<RemoteControlSidebarRow selected={false} onSelect={vi.fn()} />)
    expect(await screen.findByTitle('3 connected')).toHaveTextContent('3')
  })
})
