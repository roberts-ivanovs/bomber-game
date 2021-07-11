import React from 'react'
import { GlGames, GlLoader } from "../utils/GlLoader";
import styled, { css } from 'styled-components'


interface Props {
  title: string;
  sourcePath: GlGames;
}

const Button = styled.button<{primary?: boolean}>`
  background: transparent;
  border-radius: 3px;
  border: 2px solid palevioletred;
  color: palevioletred;
  margin: 0.5em 1em;
  padding: 0.25em 1em;

  ${props => props.primary && css`
    background: palevioletred;
    color: white;
  `}
`;

const Container = styled.div`
  text-align: center;
`
const Canvas = styled.canvas`
margin: 0px;
padding: 0px;
width: 1280px;
height: 720px;
overflow: hidden;
position: absolute;
background: black;
`

export function Game({ title, sourcePath }: Props) {
  return (
    <article>
      <h2>{title}</h2>
      <Container>
        <Button>Normal Button</Button>
        <Button primary>Primary Button</Button>
      </Container>
      <Canvas id="glcanvas" tabIndex={1}/>
      <GlLoader sourcePath={sourcePath} />
    </article>
  )
}
