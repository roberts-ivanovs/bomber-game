import React, { ReactElement, useEffect, useState } from 'react';

interface Props {
  sourcePath: GlGames;
}

// Relative to the current file
export enum GlGames {
  bomber = '/bomber.wasm'
}

export function GlLoader({ sourcePath }: Props): ReactElement {
  const [gl, setGL] = useState<any>();
  useEffect(() => {
    import('../utils/gl.js').then((glLocal) => setGL(glLocal));
  }, []);
  return <script>{gl?.load(sourcePath)}</script>;
}
