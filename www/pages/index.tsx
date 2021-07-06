import dynamic from 'next/dynamic'
import React, { ReactElement } from 'react';

import { Game } from '../components/Game';
import { GlGames } from "../utils/GlLoader";
import { HeadFilled } from "./Head";

export default function IndexPage(): ReactElement {
  return (
    <main>
      <HeadFilled />
      <h1>Title</h1>
      <section>
        <Game sourcePath={GlGames.bomber} title="Ether Bomber" />
      </section>
    </main>
  );
}
