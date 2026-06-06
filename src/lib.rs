//! # agent-motif
//!
//! Motivic development for multi-agent systems. A motif is a short recurring
//! pattern — a seed that grows. Beethoven built entire symphonies from 4-note
//! motifs. Agents can do the same: identify, transform, and develop patterns
//! into coherent structures.

use std::fmt;

/// A short musical/behavioral pattern that serves as a building block.
/// In multi-agent terms, a motif is a recognizable pattern of agent behavior
/// that can be transformed, combined, and developed into larger structures.
#[derive(Debug, Clone, PartialEq)]
pub struct Motif {
    /// The sequence of values (e.g., pitch intervals, action codes).
    pub elements: Vec<i32>,
    /// A human-readable name/label.
    pub name: String,
    /// An optional unique identifier.
    pub id: Option<String>,
}

impl Motif {
    /// Create a new motif from a sequence of values.
    pub fn new(elements: Vec<i32>, name: &str) -> Self {
        Motif {
            elements,
            name: name.to_string(),
            id: None,
        }
    }

    /// Create a motif with an ID.
    pub fn with_id(elements: Vec<i32>, name: &str, id: &str) -> Self {
        Motif {
            elements,
            name: name.to_string(),
            id: Some(id.to_string()),
        }
    }

    /// Number of elements in the motif.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Whether the motif is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get the interval between consecutive elements (as deltas).
    pub fn intervals(&self) -> Vec<i32> {
        self.elements
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect()
    }

    /// Total span (range) of the motif.
    pub fn span(&self) -> i32 {
        if self.elements.is_empty() {
            return 0;
        }
        let min = *self.elements.iter().min().unwrap();
        let max = *self.elements.iter().max().unwrap();
        max - min
    }

    /// Sum of absolute intervals.
    pub fn total_motion(&self) -> i32 {
        self.intervals().iter().map(|i| i.abs()).sum()
    }

    /// Whether the motif is symmetrical (palindromic intervals).
    pub fn is_symmetrical(&self) -> bool {
        let ivs = self.intervals();
        let rev: Vec<i32> = ivs.iter().copied().rev().collect();
        ivs == rev
    }

    /// The contour of the motif: sequence of directions (up/down/same).
    pub fn contour(&self) -> Vec<Direction> {
        self.intervals()
            .iter()
            .map(|&i| if i > 0 {
                Direction::Up
            } else if i < 0 {
                Direction::Down
            } else {
                Direction::Same
            })
            .collect()
    }

    /// Compactness: ratio of unique elements to total elements.
    pub fn compactness(&self) -> f64 {
        if self.elements.is_empty() {
            return 1.0;
        }
        let unique: std::collections::HashSet<i32> = self.elements.iter().copied().collect();
        unique.len() as f64 / self.elements.len() as f64
    }

    /// Check if this motif's contour matches another motif's contour.
    pub fn contour_matches(&self, other: &Motif) -> bool {
        self.contour() == other.contour()
    }

    /// Edit distance to another motif (Levenshtein on elements).
    pub fn edit_distance(&self, other: &Motif) -> usize {
        let m = self.elements.len();
        let n = other.elements.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }

        for i in 1..=m {
            for j in 1..=n {
                let cost = if self.elements[i - 1] == other.elements[j - 1] {
                    0
                } else {
                    1
                };
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1] + cost);
            }
        }
        dp[m][n]
    }
}

impl fmt::Display for Motif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.name, self.elements)
    }
}

/// Direction of melodic/behavioral motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Same,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "↑"),
            Direction::Down => write!(f, "↓"),
            Direction::Same => write!(f, "→"),
        }
    }
}

/// Transformations that can be applied to a motif.
/// Each transformation produces a new motif from an existing one,
/// like musical development techniques.
#[derive(Debug, Clone, PartialEq)]
pub enum MotifTransform {
    /// Inversion: flip all intervals (up becomes down, down becomes up).
    Inversion,
    /// Retrograde: reverse the order of elements.
    Retrograde,
    /// Augmentation: scale intervals by a factor.
    Augmentation { factor: i32 },
    /// Diminution: compress intervals by dividing.
    Diminution { divisor: i32 },
    /// Transposition: shift all elements by a constant.
    Transposition { interval: i32 },
    /// Retrograde inversion: reverse then invert.
    RetrogradeInversion,
    /// Sequence: repeat at a different pitch level.
    Sequence { shift: i32 },
    /// Elision: remove the last element.
    Elision,
    /// Extension: add an element.
    Extension { value: i32 },
    /// Fragmentation: use only a portion of the motif.
    Fragmentation { start: usize, len: usize },
}

impl MotifTransform {
    /// Apply the transformation to a motif, producing a new motif.
    pub fn apply(&self, motif: &Motif) -> Motif {
        let elements = match self {
            MotifTransform::Inversion => {
                let intervals = motif.intervals();
                let mut result = vec![motif.elements[0]];
                for &iv in &intervals {
                    result.push(result.last().unwrap() - iv);
                }
                result
            }
            MotifTransform::Retrograde => {
                motif.elements.iter().copied().rev().collect()
            }
            MotifTransform::Augmentation { factor } => {
                let intervals = motif.intervals();
                let mut result = vec![motif.elements[0]];
                for &iv in &intervals {
                    result.push(result.last().unwrap() + iv * factor);
                }
                result
            }
            MotifTransform::Diminution { divisor } => {
                let intervals = motif.intervals();
                let mut result = vec![motif.elements[0]];
                for &iv in &intervals {
                    result.push(result.last().unwrap() + iv / divisor);
                }
                result
            }
            MotifTransform::Transposition { interval } => {
                motif.elements.iter().map(|&e| e + interval).collect()
            }
            MotifTransform::RetrogradeInversion => {
                let inverted = MotifTransform::Inversion.apply(motif);
                inverted.elements.iter().copied().rev().collect()
            }
            MotifTransform::Sequence { shift } => {
                let mut result = motif.elements.clone();
                let shifted: Vec<i32> = motif.elements.iter().map(|&e| e + shift).collect();
                result.extend(shifted);
                result
            }
            MotifTransform::Elision => {
                let mut result = motif.elements.clone();
                result.pop();
                result
            }
            MotifTransform::Extension { value } => {
                let mut result = motif.elements.clone();
                result.push(*value);
                result
            }
            MotifTransform::Fragmentation { start, len } => {
                let end = (*start + *len).min(motif.elements.len());
                motif.elements[*start..end].to_vec()
            }
        };

        let transform_name = format!("{:?}", self);
        Motif {
            elements,
            name: format!("{} ({})", motif.name, transform_name.split_whitespace().next().unwrap_or(&transform_name)),
            id: None,
        }
    }

    /// Returns a description of the transformation.
    pub fn description(&self) -> &'static str {
        match self {
            MotifTransform::Inversion => "Flip intervals: up becomes down, down becomes up",
            MotifTransform::Retrograde => "Reverse the order of all elements",
            MotifTransform::Augmentation { .. } => "Stretch intervals to make the motif longer/larger",
            MotifTransform::Diminution { .. } => "Compress intervals to make the motif shorter/smaller",
            MotifTransform::Transposition { .. } => "Shift all elements by a constant",
            MotifTransform::RetrogradeInversion => "Reverse then invert — the most transformative operation",
            MotifTransform::Sequence { .. } => "Repeat the motif at a different level",
            MotifTransform::Elision => "Remove the last element, shortening the motif",
            MotifTransform::Extension { .. } => "Add an element to the motif",
            MotifTransform::Fragmentation { .. } => "Use only a portion of the original motif",
        }
    }
}

impl fmt::Display for MotifTransform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MotifTransform::Inversion => write!(f, "Inversion"),
            MotifTransform::Retrograde => write!(f, "Retrograde"),
            MotifTransform::Augmentation { factor } => write!(f, "Augmentation(×{})", factor),
            MotifTransform::Diminution { divisor } => write!(f, "Diminution(÷{})", divisor),
            MotifTransform::Transposition { interval } => write!(f, "Transposition(+{})", interval),
            MotifTransform::RetrogradeInversion => write!(f, "RetrogradeInversion"),
            MotifTransform::Sequence { shift } => write!(f, "Sequence(+{})", shift),
            MotifTransform::Elision => write!(f, "Elision"),
            MotifTransform::Extension { value } => write!(f, "Extension(+{})", value),
            MotifTransform::Fragmentation { start, len } => {
                write!(f, "Fragmentation({}:{})", start, start + len)
            }
        }
    }
}

/// A chain of transformations applied sequentially to build a structure.
#[derive(Debug, Clone, PartialEq)]
pub struct MotifChain {
    /// The original seed motif.
    pub seed: Motif,
    /// Ordered sequence of transforms applied.
    pub transforms: Vec<MotifTransform>,
    /// The resulting motif after all transforms.
    pub result: Motif,
    /// All intermediate motifs.
    pub intermediates: Vec<Motif>,
}

impl MotifChain {
    /// Start a new chain from a seed motif.
    pub fn from_seed(seed: Motif) -> Self {
        let result = seed.clone();
        MotifChain {
            seed: seed.clone(),
            transforms: Vec::new(),
            result,
            intermediates: vec![seed],
        }
    }

    /// Add a transformation to the chain.
    pub fn then(mut self, transform: MotifTransform) -> Self {
        let new_result = transform.apply(&self.result);
        self.transforms.push(transform);
        self.intermediates.push(new_result.clone());
        self.result = new_result;
        self
    }

    /// Number of transformations in the chain.
    pub fn depth(&self) -> usize {
        self.transforms.len()
    }

    /// Whether the chain has no transforms.
    pub fn is_identity(&self) -> bool {
        self.transforms.is_empty()
    }

    /// Get the motif at a specific step in the chain.
    pub fn at_step(&self, step: usize) -> Option<&Motif> {
        self.intermediates.get(step)
    }

    /// Total element count of the final result.
    pub fn result_len(&self) -> usize {
        self.result.len()
    }

    /// Total span growth: how much the motif has grown from seed to result.
    pub fn span_growth(&self) -> f64 {
        let seed_span = self.seed.span();
        let result_span = self.result.span();
        if seed_span == 0 {
            return 0.0;
        }
        result_span as f64 / seed_span as f64
    }

    /// Complexity growth: ratio of result length to seed length.
    pub fn complexity_growth(&self) -> f64 {
        if self.seed.is_empty() {
            return 0.0;
        }
        self.result.len() as f64 / self.seed.len() as f64
    }
}

impl fmt::Display for MotifChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chain: {}", self.seed.name)?;
        for t in &self.transforms {
            write!(f, " → {}", t)?;
        }
        write!(f, " (depth: {})", self.depth())
    }
}

/// A developmental arc: how a motif grows from simple to complex over stages.
/// Models the trajectory of motivic development in a multi-agent system.
#[derive(Debug, Clone, PartialEq)]
pub struct DevelopmentalArc {
    /// The seed motif.
    pub seed: Motif,
    /// Named stages of development.
    pub stages: Vec<ArcStage>,
}

/// A single stage in a developmental arc.
#[derive(Debug, Clone, PartialEq)]
pub struct ArcStage {
    /// Stage name.
    pub name: String,
    /// The motif at this stage.
    pub motif: Motif,
    /// Complexity score (0.0–1.0).
    pub complexity: f64,
    /// Description of what changed.
    pub description: String,
}

impl DevelopmentalArc {
    /// Create a new developmental arc from a seed motif.
    pub fn new(seed: Motif) -> Self {
        DevelopmentalArc {
            seed: seed.clone(),
            stages: vec![ArcStage {
                name: "Presentation".into(),
                motif: seed,
                complexity: 0.1,
                description: "The motif is introduced in its simplest form".into(),
            }],
        }
    }

    /// Add a development stage.
    pub fn add_stage(
        &mut self,
        name: &str,
        motif: Motif,
        complexity: f64,
        description: &str,
    ) {
        self.stages.push(ArcStage {
            name: name.to_string(),
            motif,
            complexity: complexity.clamp(0.0, 1.0),
            description: description.to_string(),
        });
    }

    /// Number of stages.
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    /// Current (latest) stage.
    pub fn current_stage(&self) -> &ArcStage {
        self.stages.last().unwrap()
    }

    /// Overall arc trajectory: is complexity generally increasing?
    pub fn is_ascending(&self) -> bool {
        if self.stages.len() < 2 {
            return true;
        }
        let mut increases = 0;
        let mut decreases = 0;
        for i in 1..self.stages.len() {
            if self.stages[i].complexity > self.stages[i - 1].complexity {
                increases += 1;
            } else if self.stages[i].complexity < self.stages[i - 1].complexity {
                decreases += 1;
            }
        }
        increases >= decreases
    }

    /// Peak complexity stage.
    pub fn peak_stage(&self) -> &ArcStage {
        self.stages
            .iter()
            .max_by(|a, b| a.complexity.partial_cmp(&b.complexity).unwrap())
            .unwrap()
    }

    /// Total complexity range (peak - min).
    pub fn complexity_range(&self) -> f64 {
        let min = self
            .stages
            .iter()
            .map(|s| s.complexity)
            .fold(f64::INFINITY, f64::min);
        let max = self
            .stages
            .iter()
            .map(|s| s.complexity)
            .fold(f64::NEG_INFINITY, f64::max);
        max - min
    }

    /// Build a standard developmental arc from a motif using common transforms.
    pub fn standard_development(seed: Motif) -> Self {
        let mut arc = DevelopmentalArc::new(seed.clone());

        // Stage: Repetition
        let repeated = MotifTransform::Sequence { shift: 2 }.apply(&seed);
        arc.add_stage("Repetition", repeated, 0.2, "Repeat the motif at a new level");

        // Stage: Inversion
        let inverted = MotifTransform::Inversion.apply(&seed);
        arc.add_stage("Inversion", inverted, 0.35, "Invert the motif's intervals");

        // Stage: Fragmentation
        if seed.len() >= 3 {
            let fragmented = MotifTransform::Fragmentation { start: 0, len: 2 }.apply(&seed);
            arc.add_stage("Fragmentation", fragmented, 0.45, "Extract a fragment for development");
        }

        // Stage: Augmentation
        let augmented = MotifTransform::Augmentation { factor: 2 }.apply(&seed);
        arc.add_stage("Augmentation", augmented, 0.6, "Stretch the motif's intervals");

        // Stage: Combination
        let retrograde = MotifTransform::Retrograde.apply(&seed);
        let combined = MotifTransform::Sequence { shift: 5 }.apply(&retrograde);
        arc.add_stage("Combination", combined, 0.75, "Combine retrograde with sequencing");

        // Stage: Full development
        let full = MotifTransform::RetrogradeInversion.apply(&seed);
        let full_chain = MotifChain::from_seed(full.clone())
            .then(MotifTransform::Augmentation { factor: 2 })
            .then(MotifTransform::Sequence { shift: 7 });
        arc.add_stage("Full Development", full_chain.result, 0.9,
            "Full development: retrograde inversion, augmented and sequenced");

        arc
    }
}

impl fmt::Display for DevelopmentalArc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DevelopmentalArc: {} ({} stages)", self.seed.name, self.stage_count())?;
        for (i, stage) in self.stages.iter().enumerate() {
            writeln!(
                f,
                "  {}. {} [complexity: {:.1}] — {}",
                i + 1,
                stage.name,
                stage.complexity,
                stage.description
            )?;
        }
        Ok(())
    }
}

/// A family of related motifs derived from a common seed.
#[derive(Debug, Clone, PartialEq)]
pub struct MotifFamily {
    /// The seed motif that defines the family.
    pub seed: Motif,
    /// All motifs in the family.
    pub members: Vec<Motif>,
    /// How each member was derived.
    pub derivations: Vec<String>,
}

impl MotifFamily {
    /// Create a new family from a seed motif.
    pub fn from_seed(seed: Motif) -> Self {
        let first = seed.clone();
        MotifFamily {
            seed: seed.clone(),
            members: vec![first],
            derivations: vec!["Original".into()],
        }
    }

    /// Generate the full family using standard transformations.
    pub fn generate_full(seed: Motif) -> Self {
        let mut family = MotifFamily::from_seed(seed.clone());

        let transforms = vec![
            (MotifTransform::Inversion, "Inversion"),
            (MotifTransform::Retrograde, "Retrograde"),
            (MotifTransform::RetrogradeInversion, "Retrograde Inversion"),
            (MotifTransform::Augmentation { factor: 2 }, "Augmentation ×2"),
            (MotifTransform::Diminution { divisor: 2 }, "Diminution ÷2"),
            (MotifTransform::Transposition { interval: 7 }, "Transposition +7"),
            (MotifTransform::Transposition { interval: -5 }, "Transposition -5"),
        ];

        for (transform, label) in transforms {
            let derived = transform.apply(&seed);
            family.members.push(derived);
            family.derivations.push(label.to_string());
        }

        family
    }

    /// Number of motifs in the family.
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Whether a motif is part of this family.
    pub fn contains(&self, motif: &Motif) -> bool {
        self.members.iter().any(|m| m.elements == motif.elements)
    }

    /// Find motifs in the family with a matching contour.
    pub fn by_contour(&self, contour: &[Direction]) -> Vec<&Motif> {
        self.members
            .iter()
            .filter(|m| m.contour() == contour)
            .collect()
    }

    /// Find the family member most similar to a given motif.
    pub fn most_similar(&self, motif: &Motif) -> (usize, usize) {
        let mut best_idx = 0;
        let mut best_dist = usize::MAX;
        for (i, member) in self.members.iter().enumerate() {
            let dist = member.edit_distance(motif);
            if dist < best_dist {
                best_dist = dist;
                best_idx = i;
            }
        }
        (best_idx, best_dist)
    }

    /// Average length of family members.
    pub fn average_length(&self) -> f64 {
        if self.members.is_empty() {
            return 0.0;
        }
        self.members.iter().map(|m| m.len()).sum::<usize>() as f64 / self.members.len() as f64
    }

    /// Length range in the family.
    pub fn length_range(&self) -> (usize, usize) {
        let min = self.members.iter().map(|m| m.len()).min().unwrap_or(0);
        let max = self.members.iter().map(|m| m.len()).max().unwrap_or(0);
        (min, max)
    }
}

impl fmt::Display for MotifFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MotifFamily: {} ({} members)", self.seed.name, self.size())?;
        for (i, member) in self.members.iter().enumerate() {
            writeln!(f, "  {}. {} — {}", i + 1, member.name, self.derivations[i])?;
        }
        Ok(())
    }
}

/// Detects recurring patterns (motifs) in sequences of values.
/// Useful for finding patterns in agent behavior over time.
#[derive(Debug, Clone)]
pub struct MotifDetector {
    /// Minimum pattern length to detect.
    pub min_length: usize,
    /// Maximum pattern length to detect.
    pub max_length: usize,
    /// Minimum number of occurrences to count as a motif.
    pub min_occurrences: usize,
}

/// A detected pattern with its occurrence count and positions.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedPattern {
    /// The pattern elements.
    pub pattern: Vec<i32>,
    /// Number of times it appears.
    pub occurrences: usize,
    /// Starting positions where it was found.
    pub positions: Vec<usize>,
}

impl MotifDetector {
    /// Create a new detector with default settings.
    pub fn new() -> Self {
        MotifDetector {
            min_length: 2,
            max_length: 8,
            min_occurrences: 2,
        }
    }

    /// Create a detector with custom parameters.
    pub fn with_params(min_length: usize, max_length: usize, min_occurrences: usize) -> Self {
        MotifDetector {
            min_length,
            max_length,
            min_occurrences,
        }
    }

    /// Detect all recurring patterns in a sequence.
    pub fn detect(&self, sequence: &[i32]) -> Vec<DetectedPattern> {
        let mut patterns: Vec<DetectedPattern> = Vec::new();
        let max_len = self.max_length.min(sequence.len());

        for len in self.min_length..=max_len {
            for start in 0..=sequence.len().saturating_sub(len) {
                let candidate: Vec<i32> = sequence[start..start + len].to_vec();

                // Skip if we already found this pattern
                if patterns.iter().any(|p| p.pattern == candidate) {
                    continue;
                }

                // Count occurrences
                let mut positions = Vec::new();
                for i in 0..=sequence.len().saturating_sub(len) {
                    if sequence[i..i + len] == candidate[..] {
                        positions.push(i);
                    }
                }

                if positions.len() >= self.min_occurrences {
                    patterns.push(DetectedPattern {
                        pattern: candidate,
                        occurrences: positions.len(),
                        positions,
                    });
                }
            }
        }

        // Sort by frequency (most common first)
        patterns.sort_by(|a, b| b.occurrences.cmp(&a.occurrences));
        patterns
    }

    /// Detect only the most frequent pattern.
    pub fn detect_most_frequent(&self, sequence: &[i32]) -> Option<DetectedPattern> {
        self.detect(sequence).into_iter().next()
    }

    /// Detect patterns by contour (shape) rather than exact values.
    pub fn detect_by_contour(&self, sequence: &[i32]) -> Vec<DetectedPattern> {
        // Convert to direction sequence
        let directions: Vec<Direction> = sequence
            .windows(2)
            .map(|w| if w[1] > w[0] {
                Direction::Up
            } else if w[1] < w[0] {
                Direction::Down
            } else {
                Direction::Same
            })
            .collect();

        let mut patterns = Vec::new();
        let max_len = self.max_length.min(directions.len());

        for len in self.min_length..=max_len {
            for start in 0..=directions.len().saturating_sub(len) {
                let candidate: Vec<Direction> = directions[start..start + len].to_vec();

                // Encode direction as i32 for DetectedPattern
                let encoded: Vec<i32> = candidate
                    .iter()
                    .map(|d| match d {
                        Direction::Up => 1,
                        Direction::Down => -1,
                        Direction::Same => 0,
                    })
                    .collect();

                if patterns.iter().any(|p: &DetectedPattern| p.pattern == encoded) {
                    continue;
                }

                let mut positions = Vec::new();
                for i in 0..=directions.len().saturating_sub(len) {
                    let segment: Vec<Direction> = directions[i..i + len].to_vec();
                    if segment == candidate {
                        positions.push(i);
                    }
                }

                if positions.len() >= self.min_occurrences {
                    patterns.push(DetectedPattern {
                        pattern: encoded,
                        occurrences: positions.len(),
                        positions,
                    });
                }
            }
        }

        patterns.sort_by(|a, b| b.occurrences.cmp(&a.occurrences));
        patterns
    }
}

impl Default for MotifDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DetectedPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pattern {:?} ({} occurrences at {:?})",
            self.pattern, self.occurrences, self.positions
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motif_creation() {
        let m = Motif::new(vec![60, 62, 64, 65], "test motif");
        assert_eq!(m.len(), 4);
        assert!(!m.is_empty());
        assert_eq!(m.name, "test motif");
    }

    #[test]
    fn test_motif_with_id() {
        let m = Motif::with_id(vec![1, 2, 3], "abc", "m1");
        assert_eq!(m.id, Some("m1".to_string()));
    }

    #[test]
    fn test_motif_intervals() {
        let m = Motif::new(vec![60, 62, 64, 65], "test");
        assert_eq!(m.intervals(), vec![2, 2, 1]);
    }

    #[test]
    fn test_motif_span() {
        let m = Motif::new(vec![60, 62, 64, 65], "test");
        assert_eq!(m.span(), 5);
    }

    #[test]
    fn test_motif_total_motion() {
        let m = Motif::new(vec![60, 62, 60, 64], "test");
        assert_eq!(m.total_motion(), 8); // |2|+|-2|+|4| = 8
    }

    #[test]
    fn test_motif_symmetrical() {
        let sym = Motif::new(vec![60, 62, 65, 67], "sym");
        // intervals: [2, 3, 2] — reversed is [2, 3, 2] ✓
        assert!(sym.is_symmetrical());

        let asym = Motif::new(vec![60, 62, 65], "asym");
        assert!(!asym.is_symmetrical());
    }

    #[test]
    fn test_motif_contour() {
        let m = Motif::new(vec![60, 62, 60, 64], "test");
        assert_eq!(
            m.contour(),
            vec![Direction::Up, Direction::Down, Direction::Up]
        );
    }

    #[test]
    fn test_motif_compactness() {
        let m = Motif::new(vec![1, 1, 1], "repeated");
        assert!((m.compactness() - 1.0 / 3.0).abs() < 0.01);

        let unique = Motif::new(vec![1, 2, 3], "unique");
        assert!((unique.compactness() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_motif_contour_matches() {
        let a = Motif::new(vec![60, 62, 64], "a");
        let b = Motif::new(vec![0, 5, 10], "b");
        assert!(a.contour_matches(&b)); // both go up-up

        let c = Motif::new(vec![60, 58, 56], "c");
        assert!(!a.contour_matches(&c));
    }

    #[test]
    fn test_motif_edit_distance() {
        let a = Motif::new(vec![1, 2, 3], "a");
        let b = Motif::new(vec![1, 2, 3], "b");
        assert_eq!(a.edit_distance(&b), 0);

        let c = Motif::new(vec![1, 2, 4], "c");
        assert_eq!(a.edit_distance(&c), 1);

        let d = Motif::new(vec![1, 2], "d");
        assert_eq!(a.edit_distance(&d), 1);
    }

    #[test]
    fn test_motif_display() {
        let m = Motif::new(vec![60, 62, 64], "C major triad");
        assert!(format!("{}", m).contains("C major triad"));
    }

    #[test]
    fn test_direction_display() {
        assert_eq!(format!("{}", Direction::Up), "↑");
        assert_eq!(format!("{}", Direction::Down), "↓");
        assert_eq!(format!("{}", Direction::Same), "→");
    }

    #[test]
    fn test_transform_inversion() {
        let m = Motif::new(vec![60, 64, 67], "triad up");
        let inv = MotifTransform::Inversion.apply(&m);
        assert_eq!(inv.elements, vec![60, 56, 53]); // intervals flipped
    }

    #[test]
    fn test_transform_retrograde() {
        let m = Motif::new(vec![1, 2, 3, 4], "forward");
        let retro = MotifTransform::Retrograde.apply(&m);
        assert_eq!(retro.elements, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_transform_augmentation() {
        let m = Motif::new(vec![60, 62, 64], "small");
        let aug = MotifTransform::Augmentation { factor: 2 }.apply(&m);
        assert_eq!(aug.elements, vec![60, 64, 68]); // intervals doubled
    }

    #[test]
    fn test_transform_diminution() {
        let m = Motif::new(vec![60, 64, 68], "large");
        let dim = MotifTransform::Diminution { divisor: 2 }.apply(&m);
        assert_eq!(dim.elements, vec![60, 62, 64]); // intervals halved
    }

    #[test]
    fn test_transform_transposition() {
        let m = Motif::new(vec![60, 62, 64], "original");
        let trans = MotifTransform::Transposition { interval: 7 }.apply(&m);
        assert_eq!(trans.elements, vec![67, 69, 71]);
    }

    #[test]
    fn test_transform_retrograde_inversion() {
        let m = Motif::new(vec![60, 64, 67], "test");
        let ri = MotifTransform::RetrogradeInversion.apply(&m);
        // Invert first: 60, 56, 53; then reverse: 53, 56, 60
        assert_eq!(ri.elements, vec![53, 56, 60]);
    }

    #[test]
    fn test_transform_sequence() {
        let m = Motif::new(vec![60, 62], "pair");
        let seq = MotifTransform::Sequence { shift: 2 }.apply(&m);
        assert_eq!(seq.elements, vec![60, 62, 62, 64]);
    }

    #[test]
    fn test_transform_elision() {
        let m = Motif::new(vec![1, 2, 3], "trio");
        let el = MotifTransform::Elision.apply(&m);
        assert_eq!(el.elements, vec![1, 2]);
    }

    #[test]
    fn test_transform_extension() {
        let m = Motif::new(vec![1, 2], "duo");
        let ext = MotifTransform::Extension { value: 5 }.apply(&m);
        assert_eq!(ext.elements, vec![1, 2, 5]);
    }

    #[test]
    fn test_transform_fragmentation() {
        let m = Motif::new(vec![1, 2, 3, 4, 5], "long");
        let frag = MotifTransform::Fragmentation { start: 1, len: 3 }.apply(&m);
        assert_eq!(frag.elements, vec![2, 3, 4]);
    }

    #[test]
    fn test_transform_description() {
        assert!(MotifTransform::Inversion.description().contains("flip") || MotifTransform::Inversion.description().contains("Flip"));
    }

    #[test]
    fn test_transform_display() {
        let aug = MotifTransform::Augmentation { factor: 3 };
        assert!(format!("{}", aug).contains("3"));
    }

    #[test]
    fn test_motif_chain_basic() {
        let seed = Motif::new(vec![60, 62, 64], "seed");
        let chain = MotifChain::from_seed(seed);
        assert_eq!(chain.depth(), 0);
        assert!(chain.is_identity());
    }

    #[test]
    fn test_motif_chain_sequential() {
        let seed = Motif::new(vec![60, 62, 64], "seed");
        let chain = MotifChain::from_seed(seed)
            .then(MotifTransform::Inversion)
            .then(MotifTransform::Retrograde);
        assert_eq!(chain.depth(), 2);
        assert!(!chain.is_identity());
        assert_eq!(chain.intermediates.len(), 3);
    }

    #[test]
    fn test_motif_chain_at_step() {
        let seed = Motif::new(vec![60, 62, 64], "seed");
        let chain = MotifChain::from_seed(seed).then(MotifTransform::Retrograde);
        assert_eq!(chain.at_step(0).unwrap().elements, vec![60, 62, 64]);
        assert_eq!(chain.at_step(1).unwrap().elements, vec![64, 62, 60]);
    }

    #[test]
    fn test_motif_chain_growth() {
        let seed = Motif::new(vec![60, 64], "seed");
        let chain = MotifChain::from_seed(seed).then(MotifTransform::Augmentation { factor: 3 });
        assert!(chain.span_growth() > 1.0);
    }

    #[test]
    fn test_motif_chain_complexity_growth() {
        let seed = Motif::new(vec![60, 62], "seed");
        let chain = MotifChain::from_seed(seed)
            .then(MotifTransform::Sequence { shift: 5 });
        assert!(chain.complexity_growth() > 1.0);
    }

    #[test]
    fn test_motif_chain_display() {
        let seed = Motif::new(vec![60, 62], "seed");
        let chain = MotifChain::from_seed(seed).then(MotifTransform::Inversion);
        let s = format!("{}", chain);
        assert!(s.contains("Chain"));
        assert!(s.contains("Inversion"));
    }

    #[test]
    fn test_developmental_arc_new() {
        let seed = Motif::new(vec![60, 62, 64, 65], "test");
        let arc = DevelopmentalArc::new(seed);
        assert_eq!(arc.stage_count(), 1);
        assert_eq!(arc.current_stage().name, "Presentation");
    }

    #[test]
    fn test_developmental_arc_add_stages() {
        let seed = Motif::new(vec![60, 62, 64], "test");
        let mut arc = DevelopmentalArc::new(seed);
        let inverted = MotifTransform::Inversion.apply(&arc.seed);
        arc.add_stage("Development", inverted, 0.5, "Inverted the motif");
        assert_eq!(arc.stage_count(), 2);
    }

    #[test]
    fn test_developmental_arc_ascending() {
        let seed = Motif::new(vec![60, 62], "test");
        let mut arc = DevelopmentalArc::new(seed);
        arc.add_stage("Stage 2", Motif::new(vec![1], "s2"), 0.3, "");
        arc.add_stage("Stage 3", Motif::new(vec![1], "s3"), 0.6, "");
        arc.add_stage("Stage 4", Motif::new(vec![1], "s4"), 0.9, "");
        assert!(arc.is_ascending());
    }

    #[test]
    fn test_developmental_arc_peak() {
        let seed = Motif::new(vec![1], "seed");
        let mut arc = DevelopmentalArc::new(seed);
        arc.add_stage("Low", Motif::new(vec![1], "l"), 0.2, "");
        arc.add_stage("Peak", Motif::new(vec![1], "p"), 0.95, "");
        arc.add_stage("End", Motif::new(vec![1], "e"), 0.5, "");
        assert_eq!(arc.peak_stage().name, "Peak");
    }

    #[test]
    fn test_developmental_arc_range() {
        let seed = Motif::new(vec![1], "seed");
        let mut arc = DevelopmentalArc::new(seed);
        arc.add_stage("Low", Motif::new(vec![1], "l"), 0.1, "");
        arc.add_stage("High", Motif::new(vec![1], "h"), 0.9, "");
        assert!((arc.complexity_range() - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_standard_development() {
        let seed = Motif::new(vec![60, 62, 64, 65], "fate");
        let arc = DevelopmentalArc::standard_development(seed);
        assert!(arc.stage_count() >= 5);
        assert!(arc.is_ascending());
    }

    #[test]
    fn test_developmental_arc_display() {
        let seed = Motif::new(vec![60, 62], "test");
        let arc = DevelopmentalArc::new(seed);
        let s = format!("{}", arc);
        assert!(s.contains("DevelopmentalArc"));
        assert!(s.contains("Presentation"));
    }

    #[test]
    fn test_motif_family_from_seed() {
        let seed = Motif::new(vec![60, 62, 64], "triad");
        let family = MotifFamily::from_seed(seed);
        assert_eq!(family.size(), 1);
    }

    #[test]
    fn test_motif_family_generate_full() {
        let seed = Motif::new(vec![60, 62, 64], "triad");
        let family = MotifFamily::generate_full(seed);
        assert_eq!(family.size(), 8); // original + 7 transforms
    }

    #[test]
    fn test_motif_family_contains() {
        let seed = Motif::new(vec![60, 62, 64], "triad");
        let family = MotifFamily::generate_full(seed);
        let retro = MotifTransform::Retrograde.apply(&Motif::new(vec![60, 62, 64], "triad"));
        assert!(family.contains(&retro));
    }

    #[test]
    fn test_motif_family_by_contour() {
        let seed = Motif::new(vec![60, 62, 64], "up");
        let family = MotifFamily::generate_full(seed);
        let up_up = vec![Direction::Up, Direction::Up];
        let matches = family.by_contour(&up_up);
        assert!(!matches.is_empty());
    }

    #[test]
    fn test_motif_family_most_similar() {
        let seed = Motif::new(vec![60, 62, 64], "triad");
        let family = MotifFamily::generate_full(seed);
        let query = Motif::new(vec![60, 62, 64], "query");
        let (idx, dist) = family.most_similar(&query);
        assert_eq!(dist, 0);
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_motif_family_average_length() {
        let seed = Motif::new(vec![60, 62, 64], "triad");
        let family = MotifFamily::generate_full(seed);
        let avg = family.average_length();
        assert!(avg >= 3.0);
    }

    #[test]
    fn test_motif_family_length_range() {
        let seed = Motif::new(vec![60, 62, 64], "triad");
        let family = MotifFamily::generate_full(seed);
        let (min, max) = family.length_range();
        assert!(min <= max);
        assert!(min >= 2);
    }

    #[test]
    fn test_motif_family_display() {
        let seed = Motif::new(vec![60, 62], "pair");
        let family = MotifFamily::from_seed(seed);
        let s = format!("{}", family);
        assert!(s.contains("MotifFamily"));
    }

    #[test]
    fn test_detector_basic() {
        let detector = MotifDetector::new();
        // [1,2] appears twice
        let seq = vec![1, 2, 3, 1, 2, 4];
        let patterns = detector.detect(&seq);
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|p| p.pattern == vec![1, 2] && p.occurrences >= 2));
    }

    #[test]
    fn test_detector_most_frequent() {
        let detector = MotifDetector::new();
        let seq = vec![1, 1, 1, 1, 1];
        let best = detector.detect_most_frequent(&seq);
        assert!(best.is_some());
        assert!(best.unwrap().occurrences >= 2);
    }

    #[test]
    fn test_detector_custom_params() {
        let detector = MotifDetector::with_params(3, 4, 2);
        let seq = vec![1, 2, 3, 4, 1, 2, 3, 4];
        let patterns = detector.detect(&seq);
        assert!(patterns.iter().any(|p| p.pattern.len() >= 3));
    }

    #[test]
    fn test_detector_no_patterns() {
        let detector = MotifDetector::with_params(2, 4, 5); // very high threshold
        let seq = vec![1, 2, 3, 4, 5];
        let patterns = detector.detect(&seq);
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_detector_positions() {
        let detector = MotifDetector::with_params(2, 2, 2);
        let seq = vec![5, 5, 1, 5, 5];
        let patterns = detector.detect(&seq);
        let pair = patterns.iter().find(|p| p.pattern == vec![5, 5]);
        assert!(pair.is_some());
        let pair = pair.unwrap();
        assert!(pair.positions.contains(&0));
        assert!(pair.positions.contains(&3));
    }

    #[test]
    fn test_detector_by_contour() {
        let detector = MotifDetector::new();
        // Up-down pattern appears twice
        let seq = vec![1, 3, 2, 5, 3, 10, 4];
        let patterns = detector.detect_by_contour(&seq);
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_detected_pattern_display() {
        let p = DetectedPattern {
            pattern: vec![1, 2],
            occurrences: 3,
            positions: vec![0, 3, 6],
        };
        let s = format!("{}", p);
        assert!(s.contains("Pattern"));
        assert!(s.contains("3 occurrences"));
    }

    #[test]
    fn test_empty_motif() {
        let m = Motif::new(vec![], "empty");
        assert!(m.is_empty());
        assert_eq!(m.span(), 0);
        assert!(m.intervals().is_empty());
    }

    #[test]
    fn test_family_empty_seed() {
        let seed = Motif::new(vec![], "empty");
        let family = MotifFamily::from_seed(seed);
        assert_eq!(family.size(), 1);
    }

    #[test]
    fn test_chain_result_len() {
        let seed = Motif::new(vec![1, 2], "seed");
        let chain = MotifChain::from_seed(seed).then(MotifTransform::Extension { value: 3 });
        assert_eq!(chain.result_len(), 3);
    }
}
