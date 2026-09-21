import { CommandBox } from "@/components/command-box";
import { ComponentPreview } from "@/components/component-preview";
import { DirectionalTransition } from "@/components/directional-transition";
import { HomeCtas } from "@/components/home-ctas";
import { PageHero } from "@/components/page-hero";
import { RATATUI_COMPONENTS } from "@/constants/ratatui";
import { ROUTES } from "@/constants/routes";
import { BreadcrumbJsonLd } from "@/seo/json-ld";

export const dynamic = "force-static";
export const revalidate = false;

export default function IndexPage() {
  return (
    <>
      <BreadcrumbJsonLd items={[{ name: "Home", path: ROUTES.HOME }]} />
      <DirectionalTransition>
        <section className="container-wrapper relative">
          <div className="container flex flex-col items-center gap-4 py-16 text-center md:py-20 lg:py-24">
            <PageHero
              description={
                <>
                  Ready-to-use, customizable terminal components rendered from Rust.
                  <br className="hidden sm:block" />
                  Built for Rust. Browse the source and adapt it to your app.
                </>
              }
              descriptionClassName="max-w-2xl text-lg sm:text-xl"
              title="Beautiful terminal UIs, made simple"
              titleClassName="max-w-7xl"
            />

            <CommandBox className="mt-4 w-full max-w-xl" />

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
