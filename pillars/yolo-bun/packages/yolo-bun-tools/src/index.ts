// Outils runtime du pilier yolo-bun. Zero dependance : Bun natif uniquement.

/** Version minimale de Bun imposee par le monorepo (upstream pin). */
export const MIN_BUN_VERSION = "1.4.4";

/** Version semantique decomposee. */
export interface SemVer {
  readonly major: number;
  readonly minor: number;
  readonly patch: number;
}

/** Analyse `major.minor.patch` (suffixes de pre-release ignores). Lance si la chaine est invalide. */
export function parseVersion(input: string): SemVer {
  const match = /^v?(\d+)\.(\d+)\.(\d+)/.exec(input.trim());
  if (match === null) {
    throw new TypeError(`Version invalide : "${input}"`);
  }
  const [, major, minor, patch] = match;
  return {
    major: Number(major),
    minor: Number(minor),
    patch: Number(patch),
  };
}

/** Compare deux versions : negatif si a < b, 0 si egales, positif si a > b. */
export function compareVersions(a: SemVer, b: SemVer): number {
  if (a.major !== b.major) return a.major - b.major;
  if (a.minor !== b.minor) return a.minor - b.minor;
  return a.patch - b.patch;
}

/** Version de Bun qui execute ce module. */
export function bunVersion(): string {
  return Bun.version;
}

/** Indique si la version `actual` satisfait le minimum `minimum`. */
export function satisfiesMinimum(actual: string, minimum: string = MIN_BUN_VERSION): boolean {
  return compareVersions(parseVersion(actual), parseVersion(minimum)) >= 0;
}

/** Verifie que le runtime courant respecte le pin du monorepo ; lance sinon. */
export function assertBunVersion(minimum: string = MIN_BUN_VERSION): void {
  const actual = bunVersion();
  if (!satisfiesMinimum(actual, minimum)) {
    throw new Error(`Bun ${actual} detecte, ${minimum} ou plus recent requis.`);
  }
}
