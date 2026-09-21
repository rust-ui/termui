import { LINK } from "@/constants/links";

export const SiteFooter = () => (
  <footer className="border-grid border-t py-6 md:py-0">
    <div className="container-wrapper">
      <div className="container flex h-14 items-center justify-between text-sm">
        <p className="text-muted-foreground">
          Built with Rust and Ratatui. The source code is available on{" "}
          <a
            href={LINK.GITHUB}
            target="_blank"
            rel="noopener"
            className="font-medium underline underline-offset-4"
          >
            GitHub
          </a>
          .
        </p>
      </div>
    </div>
  </footer>
);
