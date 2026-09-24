import { Suspense } from 'react'

import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { copyToClipboard, mcpInstall, mcpServerCommand } from '@/lib/commands'
import { renderWithQuery } from '@/test/render-with-query'

import { McpSection } from './mcp-section'

vi.mock('@/lib/commands', async () => {
  const actual = await vi.importActual<typeof import('@/lib/commands')>('@/lib/commands')
  return {
    ...actual,
    mcpServerCommand: vi.fn(),
    mcpInstall: vi.fn(),
    copyToClipboard: vi.fn(async () => {}),
  }
})

const path = '/Applications/Remote Control Conductor.app/Contents/MacOS/remote-control-conductor'

function renderSection() {
  renderWithQuery(
    <Suspense fallback={null}>
      <McpSection />
    </Suspense>,
  )
}

beforeEach(() => {
  vi.mocked(copyToClipboard).mockClear()
  vi.mocked(mcpServerCommand).mockResolvedValue({
    path,
    claudeCode: `claude mcp add --scope user remote-control-conductor -- ${path} mcp`,
    desktopJson: JSON.stringify(
      { mcpServers: { 'remote-control-conductor': { command: path, args: ['mcp'] } } },
      null,
      2,
    ),
  })
})

describe('McpSection', () => {
  it('gives the commands to set the server up by hand', async () => {
    renderSection()
    const user = userEvent.setup()
    await user.click(
      await screen.findByRole('button', {
        name: `Copy claude mcp add --scope user remote-control-conductor -- ${path} mcp`,
      }),
    )
    expect(copyToClipboard).toHaveBeenCalledWith(`claude mcp add --scope user remote-control-conductor -- ${path} mcp`)
    await user.click(screen.getByRole('button', { name: 'Copy the Claude Desktop config' }))
    expect(vi.mocked(copyToClipboard).mock.calls[1]?.[0]).toContain('"args": [\n        "mcp"')
  })

  it('adds it to every Claude profile and says how each went', async () => {
    vi.mocked(mcpInstall).mockResolvedValue([
      { profile: 'Default', desktop: { status: 'alreadyThere' }, cli: { status: 'added' } },
      {
        profile: 'Marcus1',
        desktop: { status: 'added' },
        cli: { status: 'failed', reason: "Claude Code isn't installed" },
      },
      { profile: 'CLI only', desktop: null, cli: { status: 'added' } },
    ])
    renderSection()
    await userEvent.setup().click(await screen.findByRole('button', { name: 'Add to my Claude profiles' }))
    const report = await screen.findByRole('list', { name: 'Where it was added' })
    expect(report).toHaveTextContent('Default · desktop app: already there · Claude Code: added')
    expect(report).toHaveTextContent(
      "Marcus1 · desktop app: added · Claude Code: not added: Claude Code isn't installed",
    )
    expect(report).toHaveTextContent('CLI only · Claude Code: added')
    expect(report).not.toHaveTextContent('CLI only · desktop')
    expect(screen.getByText(/Quit and reopen a Claude desktop app/)).toBeInTheDocument()
  })

  it('says so when there is no Claude profile to add it to', async () => {
    vi.mocked(mcpInstall).mockResolvedValue([])
    renderSection()
    await userEvent.setup().click(await screen.findByRole('button', { name: 'Add to my Claude profiles' }))
    expect(await screen.findByText(/no Claude profiles on this Mac/)).toBeInTheDocument()
  })
})
