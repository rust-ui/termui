import Link from "next/link";

const COMPONENTS = [
  { slug: "panel", name: "Panel", description: "Rounded bordered panel shell." },
  { slug: "key-bar", name: "Key Bar", description: "Bottom key-hint bar." },
  { slug: "select-list", name: "Select List", description: "Keyboard-navigable list." },
];

export default function Home() {
  return (
    <main className="mx-auto max-w-3xl px-6 py-16">
      <h1 className="text-4xl font-semibold">termui</h1>
      <p className="mt-2 text-muted-foreground">
        Copy-paste Ratatui components. Real Rust, rendered live in the
        browser over WebAssembly (ratzilla) — not a replayed recording.
      </p>
      <ul className="mt-10 grid gap-4 sm:grid-cols-3">
        {COMPONENTS.map((component) => (
          <li key={component.slug}>
            <Link
              href={`/docs/${component.slug}`}
              className="block rounded-lg border border-border bg-card p-4 hover:border-accent"
            >
              <div className="font-medium">{component.name}</div>
              <div className="mt-1 text-sm text-muted-foreground">
                {component.description}
              </div>
            </Link>
          </li>
        ))}
      </ul>
    </main>
  );
}
