/**
 * Small uppercase caption above a panel's content ("Featured", "Spotlight").
 * Shared by the /articles and /wip/blog highlight panels.
 */
export function PanelLabel({ children }: { children: React.ReactNode }) {
  return (
    <p className="pb-2 text-[12px] font-medium uppercase tracking-[-0.01em] text-[rgba(15,15,16,0.5)]">
      {children}
    </p>
  );
}
