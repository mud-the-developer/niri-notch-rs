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

const NotchStage: React.FC = () => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();
  const intro = spring({frame: frame - 18, fps, config: {damping: 20, stiffness: 90}});
  const topFocus = interpolate(frame, [0, 160, 260, 360], [86, 128, 132, 104], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });
  const zoom = interpolate(frame, [0, 85, 155, 230, 330], [1.25, 1.62, 2.34, 2.42, 1.74], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
    easing: Easing.inOut(Easing.cubic),
  });
  const panX = interpolate(frame, [0, 160, 250, 360], [0, 0, 4, 0], {
    extrapolateLeft: 'clamp',
    extrapolateRight: 'clamp',
  });

  const image = frame < 185 ? 'notch-compact.png' : frame < 282 ? 'notch-notification.png' : 'notch-open.png';

  return (
    <AbsoluteFill style={{alignItems: 'center', justifyContent: 'center'}}>
      <div
        style={{
          position: 'absolute',
          width: 1540,
          height: 866,
          borderRadius: 38,
          overflow: 'hidden',
          border: '1px solid rgba(255,255,255,.14)',
          background: 'rgba(255,255,255,.035)',
          boxShadow:
            '0 42px 180px rgba(0,0,0,.72), 0 0 96px rgba(53,240,180,.11)',
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
              'linear-gradient(90deg, rgba(0,0,0,.08) 0%, transparent 18%, transparent 72%, rgba(0,0,0,.58) 100%), radial-gradient(circle at 50% 0%, transparent 0%, transparent 44%, rgba(0,0,0,.18) 100%)',
          }}
        />
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

export const NiriNotchShowcase: React.FC = () => {
  return (
    <AbsoluteFill>
      <Backdrop />
      <NotchStage />
      <Title />
    </AbsoluteFill>
  );
};
