import React from 'react'
import { GlGames, GlLoader } from "../utils/GlLoader";
import style from "./Game.module.scss";

interface Props {
  title: string;
  sourcePath: GlGames;
}


export function Game({ title, sourcePath }: Props) {
  return (
    <article>
      <h2>{title}</h2>
      <canvas className={style['canvas-game']} id="glcanvas" tabIndex={1}></canvas>
      <GlLoader sourcePath={sourcePath} />
    </article>
  )
}
