# GEMINI.md — yolo-arena

## 0. Directives Fondatrices
- **NHITL (No Human In The Loop)** : execution automatisee des suites de tests et benchmarks sans supervision manuelle.
- **Zero Emoji** : conformite absolue au protocole global YOLO.
- **Transparence et Reproductibilite** : metriques objectives (tokens/sec, TTFT, perplexite, temps de recherche RAG, conformite MCP).

## 1. Role et Objectifs
Plateforme d'evaluation et de scoring comparatif pour l'ecosysteme souverain :
- Benchmarks de modeles locaux et distants.
- Evaluation de la precision des subagents et des flux automatisés de code.
- Comparatifs d'efficacite RAG entre le moteur SIMD C-ABI (`carbon_ffi`) et les solutions du marche.

## 2. Commandes Canoniques
- `bun test tests` : Verification de l'infrastructure de tests.
- `bun run src/cli.ts` : Execution complete de l'arene.
