import type { McpInstalled, McpInstallStep } from '@/lib/types'

import { useState } from 'react'

import { useMutation, useSuspenseQuery } from '@tanstack/react-query'
import { Check, Copy } from 'lucide-react'

import { Button, Skeleton } from '@/design'
import { sessionErrorMessage } from '@/features/profiles/components/session-error-message'
import { copyToClipboard, mcpInstall, mcpServerCommand } from '@/lib/commands'
import { queryKeys } from '@/lib/query/keys'

import { CopyLine } from './remote-hosts-section'

const headingClasses = 'font-mono text-[10px] font-medium uppercase tracking-[0.1em] text-muted-strong'

/**
 * The app's MCP server: Claude Desktop and Claude Code start it themselves,
 * with `<app> mcp`, to manage profiles and sessions. One button adds it to
 * every Claude profile; the commands are here to set it up by hand too.
 */
export function McpSection() {
  const { data: command } = useSuspenseQuery({
    queryKey: queryKeys.mcpCommand,
    queryFn: mcpServerCommand,
    staleTime: Infinity,
  })
  const install = useMutation({ mutationFn: mcpInstall })

  return (
    <section className="mb-8">
      <div className="mb-2.5 flex items-center justify-between">
        <span className={headingClasses}>MCP server</span>
        <Button size="sm" variant="secondary" disabled={install.isPending} onClick={() => install.mutate()}>
          {install.isPending ? 'Adding…' : 'Add to my Claude profiles'}
        </Button>
      </div>
      <div className="space-y-3 rounded-xl border border-border-soft px-4 py-3.5 text-[12.5px]">
        <p className="text-muted">
          Let Claude manage your profiles and sessions: list them, start, stop and restart sessions on your hosts, read
          a session&apos;s window, and move sessions between accounts. Claude starts the server itself, so this app
          doesn&apos;t need to be open.
        </p>
        {install.isError ? (
          <p role="alert" className="text-meta text-red">
            {sessionErrorMessage(install.error, 'It could not be added.')}
          </p>
        ) : null}
        {install.data ? <InstallReport installed={install.data} /> : null}
        <div className="space-y-1.5">
          <p className="text-muted">Or by hand, for Claude Code:</p>
          <CopyLine text={command.claudeCode} />
        </div>
        <div className="space-y-1.5">
          <p className="text-muted">
            For Claude Desktop, in <code className="font-mono text-[11.5px]">claude_desktop_config.json</code>:
          </p>
          <CopyBlock text={command.desktopJson} label="Copy the Claude Desktop config" />
        </div>
      </div>
    </section>
  )
}

function stepText(step: McpInstallStep): string {
  switch (step.status) {
    case 'added':
      return 'added'
    case 'alreadyThere':
      return 'already there'
    case 'failed':
      return `not added: ${step.reason}`
  }
}

function InstallReport({ installed }: { installed: Array<McpInstalled> }) {
  if (installed.length === 0) {
    return <p className="text-muted">There are no Claude profiles on this Mac to add it to.</p>
  }
  const addedToDesktop = installed.some((one) => one.desktop?.status === 'added')
  return (
    <div className="space-y-1">
      <ul aria-label="Where it was added" className="space-y-0.5">
        {installed.map((one) => (
          <li key={one.profile} className="text-ink-soft">
            <span className="font-medium">{one.profile}</span>
            {one.desktop ? <span className="text-muted"> · desktop app: {stepText(one.desktop)}</span> : null}
            {one.cli ? <span className="text-muted"> · Claude Code: {stepText(one.cli)}</span> : null}
          </li>
        ))}
      </ul>
      {addedToDesktop ? (
        <p className="text-muted">Quit and reopen a Claude desktop app for it to pick the server up.</p>
      ) : null}
    </div>
  )
}

function CopyBlock({ text, label }: { text: string; label: string }) {
  const [copied, setCopied] = useState(false)
  return (
    <div className="relative rounded-md border border-border-soft">
      <pre className="overflow-x-auto px-2.5 py-2 font-mono text-[11.5px] leading-relaxed text-muted-strong">
        {text}
      </pre>
      <button
        type="button"
        aria-label={copied ? 'Copied' : label}
        className="absolute top-2 right-2 cursor-pointer text-muted hover:text-ink-soft"
        onClick={async () => {
          await copyToClipboard(text)
          setCopied(true)
        }}
      >
        {copied ? <Check aria-hidden className="h-3.5 w-3.5" /> : <Copy aria-hidden className="h-3.5 w-3.5" />}
      </button>
    </div>
  )
}

export function McpSectionFallback() {
  return (
    <section className="mb-8">
      <div className="mb-2.5 flex items-center justify-between">
        <span className={headingClasses}>MCP server</span>
        <Skeleton className="h-7 w-[164px] rounded-md" />
      </div>
      <Skeleton className="h-[180px] w-full rounded-xl" />
    </section>
  )
}
