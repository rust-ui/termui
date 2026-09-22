import * as React from "react";

/**
 * Tracks whether a horizontal scroller is pinned to its left / right extreme,
 * for disabling the matching arrow button (Rustify's carousel disables the
 * native <button> at each end). Callback-ref style like `useHorizontalOverflow`
 * so it re-measures when the node mounts/changes; subscribes to the rail's own
 * `scroll` plus `ResizeObserver` + window `resize` for layout shifts.
 *
 * `atStart` / `atEnd` are both true when the content doesn't overflow.
 */
export function useRailEdges<T extends HTMLElement>() {
  const [node, setNode] = React.useState<T | null>(null);
  const [edges, setEdges] = React.useState({ atStart: true, atEnd: true });

  React.useEffect(() => {
    if (!node) {
      return;
    }

    const update = () => {
      const max = node.scrollWidth - node.clientWidth;
      setEdges({
        atStart: node.scrollLeft <= 1,
        atEnd: node.scrollLeft >= max - 1,
      });
    };

    update();

    node.addEventListener("scroll", update, { passive: true });
    const resizeObserver = new ResizeObserver(update);
    resizeObserver.observe(node);
    window.addEventListener("resize", update);

    return () => {
      node.removeEventListener("scroll", update);
      resizeObserver.disconnect();
      window.removeEventListener("resize", update);
    };
  }, [node]);

  return { ref: setNode, node, atStart: edges.atStart, atEnd: edges.atEnd };
}
