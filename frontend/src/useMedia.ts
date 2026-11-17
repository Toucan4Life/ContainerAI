import { useSyncExternalStore } from "react";

export function useMediaQuery(query: string): boolean {
  return useSyncExternalStore(
    (notify) => {
      const list = window.matchMedia(query);
      list.addEventListener("change", notify);
      return () => list.removeEventListener("change", notify);
    },
    () => window.matchMedia(query).matches,
    () => false,
  );
}

/** Phones in portrait get their own single-column layout. Keep in sync with the media queries in mobile.css. */
export const useIsMobile = () => useMediaQuery("(max-width: 720px)");
