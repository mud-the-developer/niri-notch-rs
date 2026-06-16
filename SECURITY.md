# Security Policy

## Supported versions

This project is currently pre-1.0. Security fixes target the `main` branch until tagged releases begin.

## Reporting a vulnerability

Please do not open a public issue for sensitive security reports.

Use GitHub private vulnerability reporting if available, or contact the maintainer through the GitHub profile for `mud-the-developer`.

## Security posture

`niri-notch-rs` is a local desktop companion daemon. It should not execute arbitrary AI- or user-generated shell commands. Configuration writes should be:

- scoped to the user's Niri config,
- backed up before modification,
- idempotent,
- covered by tests where possible.
