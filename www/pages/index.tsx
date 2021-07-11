import dynamic from 'next/dynamic'
import React, { ReactElement } from 'react';

import { Game } from '../components/Game';
import { GlGames } from "../utils/GlLoader";
import styled, { css } from 'styled-components'
import { HeadFilled } from "./Head";

const Container = styled.div<{ direction: 'row' |'column'}>`
  display: flex;
  justify-content: center;
  ${props => css`
    flex-direction: ${props.direction};
  `}
`

export default function IndexPage(): ReactElement {
  return (
    <main>
      <HeadFilled />
      <Container direction={'column'}>
        <h1>Title</h1>
        <section>
          <Game sourcePath={GlGames.bomber} title="Ether Bomber" />
        </section>
      </Container>
    </main>
  );
}
