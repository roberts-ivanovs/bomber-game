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
  const [jsUtuls, setJSutils] = useState<any>();
  useEffect(() => {
    import('./gl.js').then(async (glLocal) => {
      const jsUtilsLocal = await (import("./sapp_jsutils"));
      // Register a custom callback function
      const register_plugin = function (importObject: any) {
        importObject.env.console_log_unsafe = function (toLog: number) {
          console.log(jsUtilsLocal.consume_js_object(toLog));
        };
      };
      glLocal.miniquad_add_plugin({register_plugin});
      setGL(glLocal);
      setJSutils(jsUtilsLocal);
    });
  }, []);

  return <script>{gl?.load(sourcePath)}</script>;
}
