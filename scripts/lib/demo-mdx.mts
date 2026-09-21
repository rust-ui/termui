export interface MdxDemoReference {
  tag: "RustDemo" | "TerminalFrame";
  name?: string;
  src?: string;
}

/** Read the string props used by the Rust demo build scripts from MDX tags. */
export function collectMdxDemoReferences(source: string): MdxDemoReference[] {
  return [...source.matchAll(/<(RustDemo|TerminalFrame)\b([\s\S]*?)\/>/g)].map(
    ([, tag, attributes]) => {
      const props = new Map(
        [...attributes.matchAll(/\b(name|src)\s*=\s*["']([^"']+)["']/g)].map(
          ([, key, value]) => [key, value]
        )
      );

      return {
        tag: tag as MdxDemoReference["tag"],
        name: props.get("name"),
        src: props.get("src"),
      };
    }
  );
}
