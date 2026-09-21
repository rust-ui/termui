import { SiteFooter } from "@/domains/site/components/site-footer";
import { SiteHeader } from "@/domains/site/components/site-header";
import { WebMcpTools } from "@/domains/discovery/components/web-mcp-tools";

export default function AppLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="bg-background relative flex min-h-svh flex-col">
      <WebMcpTools />
      <SiteHeader />
      <main className="flex flex-1 flex-col">{children}</main>
      <SiteFooter />
    </div>
  );
}
