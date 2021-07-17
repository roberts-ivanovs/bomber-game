import React, { ReactElement, useEffect, useMemo, useState } from 'react';

interface Props {
  sourcePath: GlGames;
}

// Relative to the current file
export enum GlGames {
  bomber = '/bomber.wasm',
}

export function GlLoader({ sourcePath }: Props): ReactElement {
  // const [gl, setGL] = useState<any>();
  // const [jsUtuls, setJSutils] = useState<any>();
  useEffect(() => {
    import('./gl.js').then(async (glLocal) => {
      // Register a custom callback function
      const register_plugin = function (importObject: any) {
        importObject.env.console_log_unsafe = function (objId: number) {
          console.log(glLocal.consume_js_object(objId));
        };
      };
      glLocal.miniquad_add_plugin({
        register_plugin,
        name: 'bomber-interop',
        version: '1',
      });
      glLocal.load(sourcePath);
    });
  }, [sourcePath]);
  return <></>;
}
