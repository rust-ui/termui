import { SiteFooter } from "@/domains/site/components/site-footer";
import { SiteHeader } from "@/domains/site/components/site-header";

export default function AuthorsLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <div className="min-h-screen overflow-x-clip bg-[#fbfbfb] text-[#0f0f10] dark:bg-background dark:text-foreground">
      <SiteHeader />
      <main>{children}</main>
      <SiteFooter />
    </div>
  );
}
