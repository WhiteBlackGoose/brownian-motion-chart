<p align="center">
<h1 align="center">Brownian motion chart</h1>
</p>
<p align="center">
<i>Made using FOSS-only</i><br><br>
<img src="https://raw.githubusercontent.com/WhiteBlackGoose/WhiteBlackGoose/da3585124a9f0d585fedcec542570e38e016df4e/media/neovim.svg" width=60 height=60 /> <img src="https://cdn.svgporn.com/logos/rust.svg?response-content-disposition=attachment%3Bfilename%3Drust.svg" width=60 height=60 /> <img src="https://cdn.svgporn.com/logos/debian.svg?response-content-disposition=attachment%3Bfilename%3Ddebian.svg" width=60 height=60 /> <img src="https://cdn.svgporn.com/logos/terminal.svg?response-content-disposition=attachment%3Bfilename%3Dterminal.svg" width=60 height=60 />
</p>

![gif with demo](./demo.gif)

### What's going on

It's an infinite tape of visualisation of a value $v$, whose increment over time is defined as
$$\delta v_t \sim \mathcal{N}(0, t)$$

It's a special case of Martingale's process, for which
$$E(X_{n+1} | {\tau}_n) = E(X_n)$$

Or, in other words, no matter at which value $v$ we're right now, the expected value at any future point is equal to $v$.

