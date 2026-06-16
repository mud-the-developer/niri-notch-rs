import React from 'react';
import {
  AbsoluteFill,
  Easing,
  Img,
  Sequence,
  interpolate,
  spring,
  staticFile,
  useCurrentFrame,
  useVideoConfig,
} from 'remotion';

type ShotProps = {
  src: string;
  caption: string;
  kicker: string;
  start: number;
  duration: number;
  scale?: number;
  x?: number;
  y?: number;
};

const emerald = '#35f0b4';
const cyan = '#5bdcff';
const bg = '#03080d';

const scanlines = {
  backgroundImage:
    'linear-gradient(rgba(255,255,255,0.035) 1px, transparent 1px)',
  backgroundSize: '100% 6px',
  mixBlendMode: 'screen' as const,
  opacity: 0.32,
};

const Backdrop: React.FC = () => {
  const frame = useCurrentFrame();
  const drift = interpolate(frame, [0, 510], [0, 90]);
  return (
    <AbsoluteFill style={{backgroundColor: bg, overflow: 'hidden'}}>
      <div
        style={{
          position: 'absolute',
          inset: -240,
          background:
            `radial-gradient(circle at ${20 + drift / 8}% 20%, rgba(53,240,180,.28), transparent 30%),` +
            `radial-gradient(circle at ${80 - drift / 12}% 72%, rgba(91,220,255,.22), transparent 34%),` +
            'linear-gradient(135deg, rgba(8,24,30,1), rgba(1,5,10,1))',
          filter: 'blur(4px)',
        }}
      />
      <div style={{position: 'absolute', inset: 0, ...scanlines}} />
      <div
        style={{
          position: 'absolute',
          inset: 52,
          border: '1px solid rgba(91,220,255,.16)',
          borderRadius: 34,
          boxShadow: '0 0 70px rgba(53,240,180,.08) inset',
        }}
      />
    </AbsoluteFill>
  );
};

const Title: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();
  const pop = spring({frame, fps, config: {damping: 18, stiffness: 90}});
  const glow = interpolate(frame, [0, 50, 100], [0.4, 1, 0.72], {
    extrapolateRight: 'clamp',
  });
  return (
    <Sequence durationInFrames={120}>
      <AbsoluteFill style={{alignItems: 'center', justifyContent: 'center'}}>
        <div
          style={{
            transform: `scale(${0.88 + pop * 0.12})`,
            textAlign: 'center',
            letterSpacing: -4,
          }}
        >
          <div
            style={{
              color: 'rgba(255,255,255,.72)',
              fontSize: 34,
              letterSpacing: 10,
              textTransform: 'uppercase',
              marginBottom: 18,
            }}
          >
            Niri companion daemon
          </div>
          <div
            style={{
              color: 'white',
              fontSize: 128,
              fontWeight: 900,
              textShadow: `0 0 ${50 * glow}px rgba(53,240,180,.58)`,
            }}
          >
            niri-notch-rs
          </div>
          <div
            style={{
              marginTop: 28,
              display: 'inline-flex',
              gap: 16,
              color: '#031014',
              fontSize: 30,
              fontWeight: 800,
            }}
          >
            {['GTK3', 'layer-shell', 'Waybar rails', 'Remotion'].map((tag) => (
              <span
                key={tag}
                style={{
                  background: `linear-gradient(135deg, ${emerald}, ${cyan})`,
                  borderRadius: 999,
                  padding: '12px 22px',
                  boxShadow: '0 12px 50px rgba(53,240,180,.24)',
                }}
              >
                {tag}
              </span>
            ))}
          </div>
        </div>
      </AbsoluteFill>
    </Sequence>
  );
};

const FramedShot: React.FC<ShotProps> = ({
  src,
  caption,
  kicker,
  start,
  duration,
  scale = 1,
  x = 0,
  y = 0,
}) => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();
  const local = frame - start;
  const enter = spring({frame: local, fps, config: {damping: 22, stiffness: 110}});
  const exit = interpolate(local, [duration - 35, duration], [1, 0], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
    easing: Easing.in(Easing.cubic),
  });
  const progress = Math.max(0, Math.min(1, enter)) * exit;
  const pan = interpolate(local, [0, duration], [-16, 16], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });

  return (
    <Sequence from={start} durationInFrames={duration}>
      <AbsoluteFill style={{alignItems: 'center', justifyContent: 'center'}}>
        <div
          style={{
            width: 1450,
            height: 815,
            borderRadius: 38,
            overflow: 'hidden',
            border: '1px solid rgba(255,255,255,.2)',
            background: 'rgba(255,255,255,.05)',
            boxShadow:
              '0 45px 160px rgba(0,0,0,.62), 0 0 80px rgba(53,240,180,.16)',
            transform: `translate(${x}px, ${y + (1 - progress) * 90}px) scale(${(0.92 + progress * 0.08) * scale})`,
            opacity: progress,
          }}
        >
          <Img
            src={staticFile(`assets/${src}`)}
            style={{
              width: '100%',
              height: '100%',
              objectFit: 'cover',
              transform: `scale(1.035) translateY(${pan}px)`,
              filter: 'saturate(1.08) contrast(1.04)',
            }}
          />
        </div>
        <div
          style={{
            position: 'absolute',
            left: 150,
            top: 92,
            opacity: progress,
            transform: `translateY(${(1 - progress) * -24}px)`,
            padding: '22px 28px',
            borderRadius: 26,
            background: 'linear-gradient(135deg, rgba(0,12,16,.86), rgba(2,22,26,.62))',
            border: '1px solid rgba(91,220,255,.22)',
            boxShadow: '0 24px 80px rgba(0,0,0,.46), 0 0 36px rgba(53,240,180,.14)',
            backdropFilter: 'blur(14px)',
          }}
        >
          <div
            style={{
              color: emerald,
              fontSize: 24,
              fontWeight: 900,
              letterSpacing: 7,
              textTransform: 'uppercase',
            }}
          >
            {kicker}
          </div>
          <div
            style={{
              marginTop: 8,
              color: 'white',
              fontSize: 48,
              fontWeight: 900,
              letterSpacing: -2,
              textShadow: '0 0 32px rgba(91,220,255,.24)',
            }}
          >
            {caption}
          </div>
        </div>
      </AbsoluteFill>
    </Sequence>
  );
};

const CodeCard: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();
  const pop = spring({frame: frame - 390, fps, config: {damping: 20, stiffness: 100}});
  const lines = [
    'cargo test --locked',
    'cargo clippy --locked --all-targets --all-features -- -D warnings',
    'niri-notch-rs doctor --json',
    'niri-notch-rs install --user',
  ];
  return (
    <Sequence from={390} durationInFrames={120}>
      <AbsoluteFill style={{alignItems: 'center', justifyContent: 'center'}}>
        <div
          style={{
            width: 1320,
            borderRadius: 32,
            padding: 44,
            background: 'rgba(0,12,16,.78)',
            border: '1px solid rgba(91,220,255,.22)',
            boxShadow: '0 34px 120px rgba(0,0,0,.66)',
            transform: `translateY(${(1 - pop) * 80}px) scale(${0.95 + pop * 0.05})`,
            opacity: pop,
          }}
        >
          <div style={{color: emerald, fontSize: 30, fontWeight: 900}}>
            OSS-grade, not screenshotware
          </div>
          <div style={{marginTop: 26, fontFamily: 'monospace', fontSize: 34}}>
            {lines.map((line, index) => (
              <div
                key={line}
                style={{
                  color: index === 2 ? cyan : 'rgba(255,255,255,.86)',
                  marginTop: 18,
                }}
              >
                <span style={{color: emerald}}>$</span> {line}
              </div>
            ))}
          </div>
        </div>
      </AbsoluteFill>
    </Sequence>
  );
};

export const NiriNotchShowcase: React.FC = () => {
  return (
    <AbsoluteFill>
      <Backdrop />
      <Title />
      <FramedShot
        src="github-remotion.png"
        kicker="built with remotion"
        caption="Programmatic video pipeline"
        start={95}
        duration={115}
        scale={0.96}
      />
      <FramedShot
        src="github-niri-notch-rs.png"
        kicker="public rust repo"
        caption="CI, docs, Dependabot, packaging"
        start={180}
        duration={120}
        scale={0.96}
      />
      <FramedShot
        src="notch-open.png"
        kicker="real niri desktop capture"
        caption="Layer-shell notch, Waybar as rails"
        start={270}
        duration={115}
      />
      <FramedShot
        src="notch-notification.png"
        kicker="notification moment"
        caption="mako + media + workspace context"
        start={335}
        duration={110}
        scale={1.02}
      />
      <CodeCard />
    </AbsoluteFill>
  );
};
