import Image from "next/image";

/**
 * Content card thumbnail — shared by articles now, other content types later.
 *
 * Almost none of our content carries a real image: `frontmatter.image` is only
 * set for hand-made hero art. When it is absent we render a dependency-free DOM
 * panel: near-black background, the short thumbnail label centered in bold
 * white, nothing else. Doing it in the DOM (rather than pointing an <img> at a
 * generated PNG) means no next/og wasm and no extra request per card, and it
 * scales cleanly to the three card sizes.
 *
 * A real `frontmatter.image` (hand-made hero art) always wins and renders as a
 * normal optimised <Image>.
 *
 * SEO/social note: the crawlable per-article image is the real
 * `app/(seo)/articles/[slug]/opengraph-image.tsx` (og:image / twitter:image /
 * JSON-LD). This component is only the on-page card visual; every page under
 * /wip is noindex anyway.
 *
 * Fills its positioned parent (parent must be `relative` + set the aspect +
 * `overflow-hidden` + rounding).
 *
 * The dark DOM panel carries `data-header-invert` so `HeaderDarkOverlay` fades
 * the frosted bar in whenever a card scrolls under the transparent header (the
 * real-<Image> branch does not — a hand-made hero can be any brightness).
 */

type Size = "feature" | "grid" | "spot";

const SIZES_BY_SIZE: Record<Size, string> = {
  feature: "(min-width: 1024px) 620px, 100vw",
  grid: "(min-width: 1280px) 280px, (min-width: 640px) 45vw, 100vw",
  spot: "120px",
};

const FRAME = {
  background: "#0A0A0A",
  heading: "#FFFFFF",
} as const;

const FRAME_LAYOUT: Record<Size, { pad: string; title: string; clamp: number }> = {
  feature: { pad: "p-10", title: "text-[18px] leading-[1.3]", clamp: 3 },
  grid: { pad: "p-6", title: "text-[12px] leading-[1.3]", clamp: 3 },
  spot: { pad: "p-3", title: "text-[8px] leading-[1.3]", clamp: 3 },
};

function hasRealImage(image?: string): image is string {
  return Boolean(image);
}

export function ContentCardThumbnail({
  title,
  image,
  size,
  priority,
}: {
  title: string;
  image?: string;
  size: Size;
  priority?: boolean;
}) {
  if (hasRealImage(image)) {
    return (
      <Image
        src={image}
        alt={title}
        fill
        sizes={SIZES_BY_SIZE[size]}
        className="object-cover"
        priority={priority}
      />
    );
  }

  const l = FRAME_LAYOUT[size];

  return (
    <div
      aria-hidden
      data-header-invert
      className={`absolute inset-0 flex items-center justify-center text-center ${l.pad}`}
      style={{ background: FRAME.background }}
    >
      <span
        className={`font-semibold tracking-[-0.01em] ${l.title}`}
        style={{
          color: FRAME.heading,
          display: "-webkit-box",
          WebkitLineClamp: l.clamp,
          WebkitBoxOrient: "vertical",
          overflow: "hidden",
        }}
      >
        {title}
      </span>
    </div>
  );
}
