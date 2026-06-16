import React from 'react';
import {Composition} from 'remotion';
import {NiriNotchShowcase} from './showcase';

export const Root: React.FC = () => {
  return (
    <Composition
      id="NiriNotchShowcase"
      component={NiriNotchShowcase}
      durationInFrames={360}
      fps={30}
      width={1920}
      height={1080}
    />
  );
};
