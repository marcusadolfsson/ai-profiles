import type { AppMetadata } from '@/lib/types'

import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { ToastProvider } from '@/design'
import { getAppMetadata, openExternalUrl } from '@/lib/commands'
import { renderWithQuery } from '@/test/render-with-query'

import { AboutDialog } from './about-dialog'

vi.mock('@/lib/commands', () => ({ getAppMetadata: vi.fn(), openExternalUrl: vi.fn() }))

const metadata: AppMetadata = {
  name: 'ai-profiles',
  version: '1.1.0',
  description: '',
  authors: [],
  repository: null,
  homepage: null,
  license: null,
}

beforeEach(() => {
  vi.mocked(getAppMetadata).mockReset()
  vi.mocked(getAppMetadata).mockResolvedValue(metadata)
})

describe('AboutDialog', () => {
  it('opens the release notes from the version row', async () => {
    const onOpenWhatsNew = vi.fn()
    renderWithQuery(
      <ToastProvider>
        <AboutDialog open onClose={vi.fn()} onOpenWhatsNew={onOpenWhatsNew} />
      </ToastProvider>,
    )

    await userEvent.click(await screen.findByRole('button', { name: /what's new/i }))

    expect(onOpenWhatsNew).toHaveBeenCalledTimes(1)
  })

  it('names the repository it comes from, and credits the original', async () => {
    vi.mocked(getAppMetadata).mockResolvedValue({
      ...metadata,
      authors: ['Marcus Adolfsson', 'Bartek Czyż <bartek@czyz.it>'],
      repository: 'https://github.com/marcusadolfsson/remote-control-conductor',
    })
    renderWithQuery(
      <ToastProvider>
        <AboutDialog open onClose={vi.fn()} onOpenWhatsNew={vi.fn()} />
      </ToastProvider>,
    )

    expect(await screen.findByText('Marcus Adolfsson, Bartek Czyż')).toBeInTheDocument()
    expect(screen.getByText('github.com/marcusadolfsson/remote-control-conductor')).toBeInTheDocument()
    await userEvent.click(screen.getByRole('button', { name: /ai-profiles by Bartek Czyż/ }))
    expect(openExternalUrl).toHaveBeenCalledWith('https://github.com/bartekczyz/ai-profiles')
  })
})
