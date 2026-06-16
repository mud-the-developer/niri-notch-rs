# niri-notch-rs Remotion video

Programmatic promo video built with [Remotion](https://github.com/remotion-dev/remotion).

## Render

This repo keeps Node local to the workstation; if Node is installed globally, the `PATH` prefix is unnecessary.

```bash
export PATH="$HOME/.local/opt/node/bin:$PATH"
cd video
npm install --include=dev
npm run typecheck
npm run still
npm run render
```

Output:

```text
video/dist/niri-notch-rs-showcase.mp4
```

## Assets

The video intentionally mixes:

- real GitHub screenshots of this repo,
- a real GitHub screenshot of Remotion,
- Niri desktop smoke-test screenshots of the notch.
