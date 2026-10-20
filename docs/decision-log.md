# Decision log

## 2026-10-20
- Agent: generated GitHub Actions workflow (tauri-action, 3 OS matrix)
- Reviewed/fixed: CI failed on release job's release-upload step (all 4 OS matrix builds) because default GITHUB_TOKEN is read-only and tauri-action needs write access to create the GitHub release; fixed by adding `permissions: contents: write` to release.yml
- Wrote by hand: —

## 2026-10-16
- Agent: scaffolded get_disk_space command and registration
- Reviewed/fixed: attach debugger couldn't find process; switched to launch config
- Wrote by hand: get_disk_space logic (Option → Result with ok_or + ?)