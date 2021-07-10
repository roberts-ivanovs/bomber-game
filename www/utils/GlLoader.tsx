import React, { ReactElement, useEffect, useState } from 'react';

interface Props {
  sourcePath: GlGames;
}

// Relative to the current file
export enum GlGames {
  bomber = '/bomber.wasm',
}

export function GlLoader({ sourcePath }: Props): ReactElement {
  const [gl, setGL] = useState<any>();
  useEffect(() => {
    import('../utils/gl.js').then((glLocal) => {

      // Register a custom callback function
      const register_plugin = function (importObject: any) {
        importObject.env.hi_from_js = function (js_object: any) {
          console.log('hi');
        };
      };
      glLocal.miniquad_add_plugin({register_plugin});
      setGL(glLocal);
    });
  }, []);

  return <script>{gl?.load(sourcePath)}</script>;
}
