<!-- Code from https://github.com/Schum123/svelte-loading-spinners/blob/master/src/lib/SpinLine.svelte and refactored for svelte 5-->

<script lang="ts">
  interface Props {
    color?: string
    unit?: string
    duration?: string
    size?: string | number
    stroke?: string | number
    pause?: boolean
  }

  let {
    color = '#FF3E00',
    unit = 'px',
    duration = '4s',
    size = '60',
    stroke = '5',
    pause = false,
  }: Props = $props()
</script>

<div
  class="wrapper"
  style="--size: {size}{unit}; --color: {color}; --stroke: {stroke}{unit}; --floatSize: {size}; --duration: {duration}"
>
  <div class="line" class:pause-animation={pause}></div>
</div>

<style>
  .wrapper {
    width: var(--size);
    height: var(--stroke);
    transform: scale(calc(var(--floatSize) / 75));
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .line {
    width: var(--size);
    height: var(--stroke);
    background: var(--color);
    border-radius: var(--stroke);
    transform-origin: center center;
    animation: spineLine var(--duration) ease infinite;
  }
  .pause-animation {
    animation-play-state: paused;
  }
  @keyframes spineLine {
    0% {
      transform: rotate(-20deg);
      height: 5px;
      width: 75px;
    }
    5% {
      height: 5px;
      width: 75px;
    }
    30% {
      transform: rotate(380deg);
      height: 5px;
      width: 75px;
    }
    40% {
      transform: rotate(360deg);
      height: 5px;
      width: 75px;
    }
    55% {
      transform: rotate(0deg);
      height: 5px;
      width: 5px;
    }
    65% {
      transform: rotate(0deg);
      height: 5px;
      width: 85px;
    }
    68% {
      transform: rotate(0deg);
      height: 5px;
    }
    75% {
      transform: rotate(0deg);
      height: 5px;
      width: 1px;
    }
    78% {
      height: 5px;
      width: 5px;
    }
    90% {
      height: 5px;
      width: 75px;
      transform: rotate(0deg);
    }
    99%,
    100% {
      height: 5px;
      width: 75px;
      transform: rotate(-20deg);
    }
  }
</style>
