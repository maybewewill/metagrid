import { animate } from "motion";

export function prefersReducedMotion(): boolean {
  if (typeof window === "undefined" || typeof window.matchMedia !== "function") return false;
  return window.matchMedia("(prefers-reduced-motion: reduce)").matches;
}

export function hoverLift(node: HTMLElement) {
  if (prefersReducedMotion()) {
    return { destroy() {} };
  }

  function onEnter() {
    animate(node, { scale: 1.04 }, { duration: 0.15 });
  }
  function onLeave() {
    animate(node, { scale: 1 }, { duration: 0.15 });
  }

  node.addEventListener("mouseenter", onEnter);
  node.addEventListener("mouseleave", onLeave);

  return {
    destroy() {
      node.removeEventListener("mouseenter", onEnter);
      node.removeEventListener("mouseleave", onLeave);
    },
  };
}
