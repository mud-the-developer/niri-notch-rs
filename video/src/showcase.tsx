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

const emerald = '#35f0b4';
const cyan = '#5bdcff';
const violet = '#8d7dff';
const ink = '#03070b';

const clamp = (value: number) => Math.max(0, Math.min(1, value));

const fade = (frame: number, start: number, end: number) =>
  interpolate(frame, [start, end], [0, 1], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
    easing: Easing.out(Easing.cubic),
  });

const softExit = (frame: number, start: number, end: number) =>
  interpolate(frame, [start, end], [1, 0], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
    easing: Easing.in(Easing.cubic),
  });

const Backdrop: React.FC = () => {
  const frame = useCurrentFrame();
  const drift = interpolate(frame, [0, 360], [0, 1]);
  const pulse = 0.72 + Math.sin(frame / 22) * 0.08;

  return (
    <AbsoluteFill style={{backgroundColor: ink, overflow: 'hidden'}}>
      <div
        style={{
          position: 'absolute',
          inset: -180,
          background:
            `radial-gradient(circle at ${52 + drift * 6}% ${82 - drift * 10}%, rgba(53,240,180,${0.22 * pulse}), transparent 30%),` +
            `radial-gradient(circle at ${64 - drift * 10}% ${72 + drift * 2}%, rgba(91,220,255,${0.18 * pulse}), transparent 34%),` +
            'linear-gradient(135deg, #05090f 0%, #020409 52%, #061014 100%)',
          filter: 'blur(2px)',
        }}
      />
      <div
        style={{
          position: 'absolute',
          left: 180,
          right: 180,
          bottom: 128,
          height: 230,
          transform: `translateX(${Math.sin(frame / 70) * 22}px) rotate(-10deg)`,
          opacity: 0.82,
          background:
            'linear-gradient(100deg, transparent 4%, rgba(53,240,180,.02) 16%, rgba(91,220,255,.54) 38%, rgba(141,125,255,.34) 62%, transparent 78%)',
          clipPath: 'polygon(8% 70%, 74% 34%, 97% 37%, 32% 77%)',
          filter: 'blur(.2px) drop-shadow(0 0 42px rgba(91,220,255,.36))',
        }}
      />
      <div
        style={{
          position: 'absolute',
          left: 432,
          right: 432,
          bottom: 170,
          height: 2,
          transform: 'rotate(-10deg)',
          background: `linear-gradient(90deg, transparent, ${emerald}, ${cyan}, transparent)`,
          boxShadow: `0 0 32px ${cyan}`,
          opacity: 0.72,
        }}
      />
      <div
        style={{
          position: 'absolute',
          inset: 0,
          backgroundImage:
            'linear-gradient(rgba(255,255,255,.032) 1px, transparent 1px), linear-gradient(90deg, rgba(91,220,255,.026) 1px, transparent 1px)',
          backgroundSize: '100% 7px, 72px 100%',
          opacity: 0.24,
          mixBlendMode: 'screen',
        }}
      />
      <div
        style={{
          position: 'absolute',
          inset: 34,
          borderRadius: 34,
          border: '1px solid rgba(91,220,255,.12)',
          boxShadow: '0 0 90px rgba(53,240,180,.06) inset',
        }}
      />
    </AbsoluteFill>
  );
};

const GlassLabel: React.FC<{
  kicker: string;
  title: string;
  from: number;
  until?: number;
  x?: number;
  y?: number;
}> = ({kicker, title, from, until = 360, x = 92, y = 72}) => {
  const frame = useCurrentFrame();
  const enter = fade(frame, from, from + 22);
  const exit = softExit(frame, until - 22, until);
  const opacity = enter * exit;

  return (
    <div
      style={{
        position: 'absolute',
        left: x,
        top: y,
        opacity,
        transform: `translateY(${(1 - enter) * -20}px)`,
        padding: '18px 24px 20px',
        borderRadius: 24,
        background: 'linear-gradient(135deg, rgba(0,12,16,.88), rgba(4,24,31,.58))',
        border: '1px solid rgba(91,220,255,.22)',
        boxShadow: '0 18px 70px rgba(0,0,0,.46), 0 0 38px rgba(53,240,180,.14)',
        backdropFilter: 'blur(18px)',
      }}
    >
      <div
        style={{
          color: emerald,
          fontSize: 18,
          fontWeight: 900,
          letterSpacing: 8,
          textTransform: 'uppercase',
        }}
      >
        {kicker}
      </div>
      <div
        style={{
          marginTop: 7,
          color: 'white',
          fontSize: 42,
          lineHeight: 1.02,
          fontWeight: 950,
          letterSpacing: -1.8,
          textShadow: '0 0 34px rgba(91,220,255,.22)',
        }}
      >
        {title}
      </div>
    </div>
  );
};

const NotchStage: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();
  const intro = spring({frame: frame - 18, fps, config: {damping: 20, stiffness: 90}});
  const macro = fade(frame, 112, 170);
  const notify = fade(frame, 205, 245);
  const settle = fade(frame, 296, 336);
  const topFocus = interpolate(frame, [0, 160, 260, 360], [70, 112, 118, 90], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });
  const zoom = interpolate(frame, [0, 85, 155, 230, 330], [1.12, 1.34, 2.18, 2.32, 1.54], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
    easing: Easing.inOut(Easing.cubic),
  });
  const panX = interpolate(frame, [0, 160, 250, 360], [0, -8, 12, 0], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });

  const image = frame < 185 ? 'notch-compact.png' : frame < 282 ? 'notch-notification.png' : 'notch-open.png';

  return (
    <AbsoluteFill style={{alignItems: 'center', justifyContent: 'center'}}>
      <div
        style={{
          position: 'absolute',
          width: 1390,
          height: 782,
          borderRadius: 38,
          overflow: 'hidden',
          border: '1px solid rgba(255,255,255,.18)',
          background: 'rgba(255,255,255,.035)',
          boxShadow:
            '0 42px 180px rgba(0,0,0,.72), 0 0 120px rgba(53,240,180,.14)',
          transform: `translate(${panX}px, ${(1 - intro) * 90 + 18}px) scale(${0.94 + clamp(intro) * 0.06})`,
          opacity: clamp(intro),
        }}
      >
        <Img
          src={staticFile(`assets/${image}`)}
          style={{
            width: '100%',
            height: '100%',
            objectFit: 'cover',
            transform: `scale(${zoom}) translateY(${topFocus}px)`,
            transformOrigin: '50% 0%',
            filter: 'saturate(1.12) contrast(1.06) brightness(.94)',
          }}
        />
        <div
          style={{
            position: 'absolute',
            inset: 0,
            background:
              'radial-gradient(circle at 50% 0%, transparent 0%, transparent 42%, rgba(0,0,0,.20) 100%)',
          }}
        />
      </div>
      <div
        style={{
          position: 'absolute',
          top: 154,
          left: '50%',
          height: 1,
          width: interpolate(frame, [118, 174], [0, 760], {
            extrapolateLeft: 'clamp',
            extrapolateRight: 'clamp',
          }),
          transform: 'translateX(-50%)',
          background: `linear-gradient(90deg, transparent, ${emerald}, ${cyan}, transparent)`,
          opacity: macro * softExit(frame, 255, 300),
          boxShadow: `0 0 26px ${cyan}`,
        }}
      />
      <div
        style={{
          position: 'absolute',
          right: 116,
          top: 92,
          display: 'flex',
          gap: 10,
          opacity: notify * softExit(frame, 300, 340),
          transform: `translateX(${(1 - notify) * 18}px)`,
        }}
      >
        {['workspace', 'mako', 'media'].map((tag, index) => (
          <span
            key={tag}
            style={{
              color: index === 1 ? '#041014' : 'rgba(255,255,255,.88)',
              background:
                index === 1
                  ? `linear-gradient(135deg, ${emerald}, ${cyan})`
                  : 'rgba(255,255,255,.08)',
              border: '1px solid rgba(255,255,255,.16)',
              borderRadius: 999,
              padding: '10px 15px',
              fontSize: 18,
              fontWeight: 900,
              letterSpacing: -0.4,
              boxShadow: index === 1 ? '0 0 34px rgba(53,240,180,.26)' : undefined,
            }}
          >
            {tag}
          </span>
        ))}
      </div>
      <div
        style={{
          position: 'absolute',
          left: 132,
          bottom: 118,
          opacity: settle,
          color: 'rgba(255,255,255,.62)',
          fontSize: 24,
          fontWeight: 700,
          letterSpacing: -0.4,
        }}
      >
        GTK3 layer-shell overlay · Niri IPC · narrow testable effects
      </div>
    </AbsoluteFill>
  );
};

const Title: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();
  const pop = spring({frame, fps, config: {damping: 18, stiffness: 82}});
  const exit = softExit(frame, 86, 118);

  return (
    <Sequence from={0} durationInFrames={128}>
      <AbsoluteFill style={{alignItems: 'center', justifyContent: 'center'}}>
        <div
          style={{
            opacity: clamp(pop) * exit,
            transform: `translateY(${(1 - clamp(pop)) * 34}px) scale(${0.9 + clamp(pop) * 0.1})`,
            textAlign: 'center',
          }}
        >
          <div
            style={{
              color: emerald,
              fontSize: 23,
              fontWeight: 900,
              letterSpacing: 13,
              textTransform: 'uppercase',
              marginBottom: 18,
            }}
          >
            Niri companion notch
          </div>
          <div
            style={{
              color: 'white',
              fontSize: 106,
              fontWeight: 950,
              letterSpacing: -6,
              textShadow: '0 0 60px rgba(53,240,180,.42)',
            }}
          >
            niri-notch-rs
          </div>
          <div
            style={{
              marginTop: 22,
              color: 'rgba(255,255,255,.62)',
              fontSize: 30,
              fontWeight: 700,
              letterSpacing: -0.8,
            }}
          >
            a tiny layer-shell island that feels native
          </div>
        </div>
      </AbsoluteFill>
    </Sequence>
  );
};

const EndCard: React.FC = () => {
  const frame = useCurrentFrame();
  const enter = fade(frame, 318, 350);
  return (
    <div
      style={{
        position: 'absolute',
        right: 90,
        bottom: 72,
        opacity: enter,
        transform: `translateY(${(1 - enter) * 24}px)`,
        display: 'flex',
        alignItems: 'center',
        gap: 14,
        padding: '16px 20px',
        borderRadius: 999,
        background: 'rgba(0,12,16,.70)',
        border: '1px solid rgba(91,220,255,.20)',
        boxShadow: '0 20px 70px rgba(0,0,0,.44)',
        backdropFilter: 'blur(14px)',
      }}
    >
      <span
        style={{
          width: 12,
          height: 12,
          borderRadius: 999,
          background: `linear-gradient(135deg, ${emerald}, ${cyan})`,
          boxShadow: `0 0 26px ${emerald}`,
        }}
      />
      <span style={{color: 'white', fontWeight: 900, fontSize: 22}}>built in Rust</span>
      <span style={{color: 'rgba(255,255,255,.38)', fontWeight: 800, fontSize: 22}}>·</span>
      <span style={{color: cyan, fontWeight: 900, fontSize: 22}}>scripted with Remotion</span>
    </div>
  );
};

export const NiriNotchShowcase: React.FC = () => {
  return (
    <AbsoluteFill>
      <Backdrop />
      <NotchStage />
      <Title />
      <GlassLabel kicker="real desktop capture" title="small surface, crisp motion" from={118} until={206} />
      <GlassLabel kicker="notification moment" title="context without clutter" from={214} until={306} />
      <EndCard />
    </AbsoluteFill>
  );
};
