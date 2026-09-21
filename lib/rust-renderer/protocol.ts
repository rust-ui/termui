export type RustPreviewIndex = Record<string, string[]>;

export const getPreviewKey = (base: string, name: string) => `${base}/${name}`;
