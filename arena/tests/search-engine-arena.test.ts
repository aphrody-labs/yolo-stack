import { describe, expect, it } from 'bun:test';
import { existsSync } from 'node:fs';
import { pillarDir } from '../src/paths.ts';

// The search engine lives in the yolo-web pillar (pillars/yolo-web or YOLO_STACK_YOLO_WEB_DIR); the suite is skipped without it.
const web = pillarDir('yolo-web');
const enginePath = `${web}/packages/search-core/src/engine.ts`;
const available = web !== undefined && existsSync(enginePath);
const { SovereignSearchEngine } = available ? await import(enginePath) : { SovereignSearchEngine: class {} };

describe.skipIf(!available)('YOLO-Arena Suite: Sovereign Search Multi-Provider Benchmark', () => {
  const engine = new SovereignSearchEngine() as any;

  it('measures sub-millisecond local SQLite doc lookup speed', () => {
    const start = performance.now();
    for (let i = 0; i < 50; i++) {
      engine.resolveDocs({ library: 'bun', symbol: 'Bun.serve' });
    }
    const totalMs = performance.now() - start;
    const avgMs = totalMs / 50;

    expect(avgMs).toBeLessThan(1.0); // Sub-millisecond target
  });

  it('verifies cost efficiency: local search is always 0.00 USD', async () => {
    const res = await engine.search('bun', { provider: 'free-context7' });
    expect(res.costDollars).toBe(0.0);
    expect(res.hits.length).toBeGreaterThan(0);
  });

  it('ensures federated failover resiliency', async () => {
    // When requesting fastest, ensure it resolves without throwing even under edge conditions
    const res = await engine.search('test non-existent symbol ' + Date.now(), { provider: 'fastest' });
    expect(res).toBeDefined();
    expect(res.hits).toBeDefined();
  });
});
