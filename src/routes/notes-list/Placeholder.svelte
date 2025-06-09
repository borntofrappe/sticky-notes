<svg display="block" viewBox="0 0 170 170">
  <defs>
    <path
      id="placeholder-paper"
      d="M 70 27 C 95 39 123 46 147 54 144 80 125 117 110 127 81 123 44 110 30 101 44 85 61 47 70 27"
      fill="currentColor"
      stroke="currentColor"
      stroke-width="4"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
    <mask id="placeholder-mask">
      <rect opacity="0.99" fill="white" width="170" height="170" />
      <use opacity="0.25" style="color: black" href="#placeholder-paper" />
    </mask>
    <path
      id="placeholder-particle"
      d="M 0 -8 A 10 10 0 0 1 -8 0 10 10 0 0 1 0 8 10 10 0 0 1 8 0 10 10 0 0 1 0 -8"
      fill="currentColor"
      stroke="currentColor"
      stroke-width="5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </defs>
  <rect
    style="color: var(--color-background, hsl(0 0% 10%))"
    fill="currentColor"
    width="170"
    height="170"
  />

  <use
    style="color: var(--color-paper, hsl(0 0% 25%))"
    href="#placeholder-paper"
  />
  <g style="color: var(--color-particle, hsl(0 0% 80%))">
    <use
      transform="translate(48 38) rotate(-35) scale(0.8)"
      href="#placeholder-particle"
    />
    <use
      transform="translate(34 122) rotate(-40) scale(0.6)"
      href="#placeholder-particle"
    />
    <use
      transform="translate(134 100) rotate(10)"
      href="#placeholder-particle"
    />
  </g>
  <g mask="url(#placeholder-mask)">
    <g
      transform="translate(36 60) rotate(-24)"
      fill="currentColor"
      stroke="currentColor"
      stroke-width="4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path
        style="color: var(--color-pencil-handle, hsl(0 0% 60%))"
        d="M 7 0 A 7 7 0 0 0 -7 0 L -7 82 0 98 7 82 7 0"
      />
      <path
        style="color: var(--color-pencil-top, hsl(0 0% 80%))"
        d="M -7 82 0 98 7 82 Q 0 82.5 -7 82"
      />
      <path
        style="color: var(--color-pencil-tip, hsl(0 0% 60%))"
        d="M -2.2 93 0 98 2.2 93 Q 0 93.2 -2.2 93"
      />
    </g>
  </g>
</svg>
