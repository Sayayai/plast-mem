import { cwd } from 'node:process'

import { findWorkspaceDir } from '@pnpm/find-workspace-dir'

// eslint-disable-next-line antfu/no-top-level-await
export const workspaceDir = await findWorkspaceDir(cwd())
  .then(dir => dir!)
