import { ComponentPreview } from "@/components/component-preview";
import { DirectionalTransition } from "@/components/directional-transition";
import { HomeCtas } from "@/components/home-ctas";
import { PageHero } from "@/components/page-hero";
import { RATATUI_COMPONENTS } from "@/constants/ratatui";
import { ROUTES } from "@/constants/routes";
import { createPageMetadata } from "@/seo/metadata";

export const dynamic = "force-static";
export const revalidate = false;

export const metadata = createPageMetadata({
  description:
    "Build Rust terminal UIs with copyable Ratatui components. Explore buttons, inputs, dialogs, charts, and more, with Rust source and terminal previews.",
  path: ROUTES.HOME,
  title: "Ratatui UI Components for Rust Terminal Apps",
});

export default function IndexPage() {
  return (
    <>
      <DirectionalTransition>
        <section className="container-wrapper relative">
          <div className="container flex flex-col items-center gap-4 py-16 text-center md:py-20 lg:py-24">
            <PageHero
              description={
                <>
                  Copyable Ratatui widgets for Rust terminal applications.
                  <br className="hidden sm:block" />
                  Browse the Rust source and adapt each component to your app.
                </>
              }
              descriptionClassName="max-w-2xl text-lg sm:text-xl"
              title="Ratatui UI Components for Rust Terminal Apps"
              titleClassName="max-w-7xl"
            />

            <HomeCtas className="mt-4" />
          </div>
        </section>

        <section className="container-wrapper pb-8 lg:pb-12">
          <div className="container">
            <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-4">
              {RATATUI_COMPONENTS.map((item) => (
                <ComponentPreview
                  key={item.name}
                  className="mt-0 h-full"
                  title={item.title}
                  name={item.name}
                  hideCode
                />
              ))}
            </div>
          </div>
        </section>
      </DirectionalTransition>
    </>
  );
}
