# agent-motif

**Motivic development for multi-agent systems: short ideas that grow into structures.**

Beethoven built his entire Fifth Symphony from a 4-note motif: G-G-G-Eb. Through repetition, inversion, fragmentation, augmentation, and combination, that tiny seed became one of the most powerful structures in Western music. This crate models that process for multi-agent systems.

## Core Concept

A **motif** is a short, recognizable pattern — a seed that can be transformed and developed into larger structures. In multi-agent systems, motifs represent recurring behavioral patterns that agents can:

1. **Identify** — detect when a pattern repeats
2. **Transform** — invert, reverse, stretch, compress, transpose
3. **Chain** — compose transformations into developmental sequences
4. **Develop** — grow from simple to complex through an arc
5. **Detect** — find recurring patterns in agent behavior over time

## Key Types

### `Motif`
A short pattern (sequence of `i32` values) with a name and optional ID. Provides:
- **Intervals** — the deltas between consecutive elements
- **Contour** — the shape (up/down/same) independent of exact values
- **Span** — total range covered
- **Symmetry** — whether intervals are palindromic
- **Edit distance** — similarity measurement to other motifs

### `MotifTransform`
Transformations that produce new motifs from existing ones:

| Transform | Musical Effect | Agent Analogy |
|-----------|---------------|---------------|
| `Inversion` | Flip intervals (up↔down) | Reverse behavior polarity |
| `Retrograde` | Reverse order | Replay actions backwards |
| `Augmentation(factor)` | Stretch intervals | Scale up action magnitude |
| `Diminution(divisor)` | Compress intervals | Scale down action magnitude |
| `Transposition(interval)` | Shift all values | Offset all actions |
| `RetrogradeInversion` | Reverse + invert | Full perspective flip |
| `Sequence(shift)` | Repeat at new level | Replay in new context |
| `Elision` | Remove last element | Simplify |
| `Extension(value)` | Add an element | Expand |
| `Fragmentation(start, len)` | Extract a portion | Focus on a subset |

### `MotifChain`
A sequence of transformations applied to a seed motif. Tracks all intermediate results, measures growth (span growth, complexity growth), and supports step-by-step inspection.

```rust
let chain = MotifChain::from_seed(seed)
    .then(MotifTransform::Inversion)
    .then(MotifTransform::Augmentation { factor: 2 })
    .then(MotifTransform::Sequence { shift: 7 });
println!("Depth: {}, Growth: {:.1}x", chain.depth(), chain.span_growth());
```

### `DevelopmentalArc`
How a motif grows from simple to complex over named stages. Models the trajectory of motivic development:

1. **Presentation** — introduce the motif
2. **Repetition** — establish through repetition
3. **Transformation** — invert, augment, fragment
4. **Combination** — combine multiple transforms
5. **Full development** — maximum complexity

Each stage has a complexity score (0.0–1.0). The arc tracks whether development is ascending, identifies peak complexity, and measures the total complexity range.

### `MotifFamily`
A collection of related motifs derived from a common seed through standard transformations (inversion, retrograde, retrograde-inversion, augmentation, diminution, transposition). Supports:
- Membership queries
- Contour matching (find family members by shape)
- Similarity search (find the most similar family member)
- Statistics (average length, length range)

### `MotifDetector`
Finds recurring patterns in sequences of values. Useful for identifying motifs in agent behavior over time. Supports:
- Exact pattern detection with configurable min/max length and occurrence threshold
- Contour-based detection (matching shapes rather than exact values)
- Most-frequent pattern extraction
- Position tracking (where each pattern occurs)

## Usage

```rust
use agent_motif::*;

// Create a motif (Beethoven's fate motif)
let fate = Motif::new(vec![67, 67, 67, 63], "Fate");

// Build a developmental arc
let arc = DevelopmentalArc::standard_development(fate.clone());
println!("{}", arc); // Shows all stages

// Generate a full family of related motifs
let family = MotifFamily::generate_full(fate);
println!("Family size: {}", family.size());

// Detect patterns in agent behavior
let detector = MotifDetector::new();
let behavior = vec![1, 2, 1, 2, 3, 1, 2, 1, 2, 3, 4];
let patterns = detector.detect(&behavior);
for p in &patterns {
    println!("Found {:?} {} times", p.pattern, p.occurrences);
}
```

## Design Philosophy

Motivic development is one of the most powerful structural techniques in music, and it maps directly to multi-agent behavior:

1. **Seeds matter** — a good motif (like a good agent behavior) carries potential
2. **Transformation > creation** — develop existing patterns rather than inventing new ones
3. **Recognition enables coherence** — agents that detect shared patterns can coordinate
4. **Development is progressive** — structures grow through stages, not all at once
5. **Families provide vocabulary** — a seed generates a whole family of related behaviors

The key insight: complex behavior doesn't require complex origins. A simple 4-element pattern, systematically developed, can generate everything needed for rich multi-agent interaction.

## License

MIT
