# Visual thesis — the luminous review boundary

Agent Write Barrier uses a **luminous glass data landscape** in a deliberate single dark mode. The filesystem is shown as a deep, quiet field; the allowed worktree is a translucent cyan glass plane; attempted writes are warm signal traces that stop at its edge. This makes the central product idea—the review surface has a visible boundary—legible before the copy is read. It is not a generic security shield or gradient hero.

## Palette

| Token | Value | Role |
| --- | --- | --- |
| Void | `#07110f` | page background; the untrusted filesystem field |
| Deep moss | `#0b1a16` | raised background |
| Glass | `rgba(19, 42, 36, .72)` | translucent surfaces |
| Hairline | `#31564b` | borders and inactive paths |
| Chalk | `#f2f7f3` | primary text |
| Mist | `#a8beb5` | secondary text (7.4:1 on Void) |
| Signal | `#77f2c3` | allowed state and primary action |
| Signal ink | `#062017` | text on Signal |
| Amber | `#ffca72` | blocked writes and warnings |
| Coral | `#ff8d82` | destructive/error state |

The palette is derived from a terminal at night and the green-blue edge refraction of thick glass. Amber is reserved for a write that meets the boundary; status is always paired with an icon and text. The explicit single-mode treatment supports the nocturnal, observability-led thesis and avoids a cosmetic theme toggle in a documentation utility.

## Type

- **Interface / editorial:** self-hosted Manrope variable, 400–800. Its broad apertures keep dense technical copy humane.
- **Code / measurements:** system monospace (`ui-monospace`, `SFMono-Regular`, `Consolas`). No font download is required for code.
- Scale: 14 / 16 / 20 / 28 / clamp(44, 7vw, 78). Body is 16–18px with 1.6 line height; prose is capped at 68 characters.

## Spacing and composition

The base rhythm is 4px, with working intervals of 8, 12, 16, 24, 32, 48, 72, and 112px. Large stretches of empty dark field separate ideas; glass panels are used only for genuinely independent artifacts (policy, terminal receipt, guarantees). On mobile, navigation links collapse to the two task-critical actions, the hero becomes copy-first, and comparison rows become stacked definitions.

## Interaction grammar

Controls feel like boundary nodes: rounded rectangles with a 1px luminous edge, 44px minimum target, and a small directional notch or arrow. Hover raises brightness, not position. Focus uses a 3px amber outer ring. The live demo progresses from `ready` → `checking` → `blocked/reported`, and always provides a text status in a polite live region. Copy buttons replace their label briefly with “Copied”.

## Motion

Only state change moves. Hero traces reveal once over 700ms; demo receipt rows fade and translate 8px over 220ms; buttons respond over 160ms. Nothing loops. Under `prefers-reduced-motion: reduce`, transitions and transforms are removed and the final trace state renders immediately.

## Original asset plan and provenance

- `site/public/hero-boundary.webp`: generated specifically for this product with the factory image deployment, then converted locally to WebP. Prompt: “Abstract editorial 3D data landscape for a developer security CLI: a single translucent sea-green glass worktree slab floating above a near-black moss filesystem grid, tiny amber data paths approach from outside and terminate cleanly at the glowing perimeter, a few pale code-like blocks safely inside, oblique isometric view, premium technical magazine art direction, physically plausible glass refraction, deep negative space, restrained palette of #07110f #77f2c3 #ffca72, no shield, no padlock, no people, no logos, no letters, no readable text, no watermark, wide 3:2 composition.” Deployment and generation settings are recorded in the adjacent `.json` provenance file. Factory-generated for this product; no third-party source asset.
- `site/public/og-boundary.webp`: deterministic 1200 × 630 center crop of the original hero artwork. No new source material was introduced.
- `site/public/apple-touch-icon.png`: deterministic square crop of the original hero artwork at 180 × 180. No new source material was introduced.
- Interface icons and wordmark geometry are original inline SVG/CSS, built from boundary-line motifs. They are decorative where the adjacent label conveys meaning.

The hero is explanatory but non-essential: its alt text describes the relationship, and the same boundary model is stated in adjacent HTML.
