use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

// ============================================================================
// SLOW VERSION (for comparison)
// ============================================================================

fn analyze_text_slow(text: &str) -> TextStats {
    let start = Instant::now();

    // Count word frequencies (SLOW VERSION)
    let mut word_freq = HashMap::new();
    for line in text.lines() {
        for word in line.split_whitespace() {
            let clean_word = word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();

            if !clean_word.is_empty() {
                *word_freq.entry(clean_word.clone()).or_insert(0) += 1;
            }
        }
    }

    // Find most common words (SLOW VERSION - O(n²))
    let mut top_words = Vec::new();
    for _ in 0..10 {
        let mut max_word = String::new();
        let mut max_count = 0;

        for (word, count) in &word_freq {
            let mut found = false;
            for (existing_word, _) in &top_words {
                if word == existing_word {
                    found = true;
                    break;
                }
            }

            if !found && *count > max_count {
                max_word = word.clone();
                max_count = *count;
            }
        }

        if max_count > 0 {
            top_words.push((max_word, max_count));
        }
    }

    // Count characters (SLOW VERSION)
    let mut char_count = 0;
    for line in text.lines() {
        for ch in line.chars() {
            if ch.is_alphabetic() {
                char_count += 1;
            }
        }
    }

    // Find longest words (SLOW VERSION)
    let mut all_words = Vec::new();
    for line in text.lines() {
        for word in line.split_whitespace() {
            let clean = word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();
            if !clean.is_empty() {
                all_words.push(clean);
            }
        }
    }

    all_words.sort_by(|a, b| b.len().cmp(&a.len()));
    let longest_words: Vec<String> = all_words.iter()
        .take(5)
        .map(|s| s.clone())
        .collect();

    TextStats {
        word_count: word_freq.len(),
        char_count,
        top_words,
        longest_words,
        time_ms: start.elapsed().as_millis(),
    }
}

// ============================================================================
// OPTIMIZED VERSION
// ============================================================================

fn analyze_text_fast(text: &str) -> TextStats {
    let start = Instant::now();

    let mut word_freq = HashMap::new();
    let mut char_count = 0;
    let mut max_len = 0;
    
    // OPTIMIZATION 1: Single pass through the text
    // Instead of 4 separate iterations, we do everything in one pass
    for word in text.split_whitespace() {
        // Count characters while processing
        for ch in word.chars() {
            if ch.is_alphabetic() {
                char_count += 1;
            }
        }

        // OPTIMIZATION 2: Filter and lowercase in-place without allocating String
        let clean_word: String = word.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        if !clean_word.is_empty() {
            // Track max length for later
            if clean_word.len() > max_len {
                max_len = clean_word.len();
            }
            
            // OPTIMIZATION 3: Only one entry() call, no clone before insertion
            *word_freq.entry(clean_word).or_insert(0) += 1;
        }
    }

    // OPTIMIZATION 4: Use BinaryHeap for top-K (O(n log k) instead of O(n²))
    // We use Reverse to get a min-heap behavior
    use std::cmp::Reverse;
    let mut heap: BinaryHeap<Reverse<(usize, String)>> = BinaryHeap::new();
    
    for (word, count) in word_freq.iter() {
        heap.push(Reverse((*count, word.clone())));
        if heap.len() > 10 {
            heap.pop();
        }
    }

    // Extract and reverse to get descending order
    let mut top_words: Vec<(String, usize)> = heap
        .into_iter()
        .map(|Reverse((count, word))| (word, count))
        .collect();
    top_words.sort_by(|a, b| b.1.cmp(&a.1));

    // OPTIMIZATION 5: Find longest words efficiently using BinaryHeap
    let mut longest_heap: BinaryHeap<Reverse<(usize, &str)>> = BinaryHeap::new();
    
    for word in word_freq.keys() {
        longest_heap.push(Reverse((word.len(), word.as_str())));
        if longest_heap.len() > 5 {
            longest_heap.pop();
        }
    }

    let mut longest_words: Vec<String> = longest_heap
        .into_iter()
        .map(|Reverse((_, word))| word.to_string())
        .collect();
    longest_words.sort_by(|a, b| b.len().cmp(&a.len()));

    TextStats {
        word_count: word_freq.len(),
        char_count,
        top_words,
        longest_words,
        time_ms: start.elapsed().as_millis(),
    }
}

// ============================================================================
// SHARED STRUCTURES
// ============================================================================

#[derive(Debug)]
struct TextStats {
    word_count: usize,
    char_count: usize,
    top_words: Vec<(String, usize)>,
    longest_words: Vec<String>,
    time_ms: u128,
}

fn generate_test_text(size: usize) -> String {
    let words = vec!["rust", "performance", "optimization", "memory", "speed",
                     "efficiency", "benchmark", "algorithm", "data", "structure"];

    (0..size)
        .map(|i| words[i % words.len()])
        .collect::<Vec<_>>()
        .join(" ")
}

// ============================================================================
// MAIN - BENCHMARK BOTH VERSIONS
// ============================================================================

fn main() {
    let text = generate_test_text(50_000);
    println!("📊 Text Analyzer Performance Comparison");
    println!("Analyzing {} bytes of text...\n", text.len());

    // Run slow version
    println!("🐌 SLOW VERSION (Baseline)");
    println!("{}", "=".repeat(50));
    let stats_slow = analyze_text_slow(&text);
    println!("Results:");
    println!("  Unique words: {}", stats_slow.word_count);
    println!("  Total chars: {}", stats_slow.char_count);
    println!("  Top 10 words: {:?}", &stats_slow.top_words[..3.min(stats_slow.top_words.len())]);
    println!("  Longest words: {:?}", &stats_slow.longest_words[..3.min(stats_slow.longest_words.len())]);
    println!("⏱️  Time: {} ms\n", stats_slow.time_ms);

    // Run fast version
    println!("⚡ OPTIMIZED VERSION");
    println!("{}", "=".repeat(50));
    let stats_fast = analyze_text_fast(&text);
    println!("Results:");
    println!("  Unique words: {}", stats_fast.word_count);
    println!("  Total chars: {}", stats_fast.char_count);
    println!("  Top 10 words: {:?}", &stats_fast.top_words[..3.min(stats_fast.top_words.len())]);
    println!("  Longest words: {:?}", &stats_fast.longest_words[..3.min(stats_fast.longest_words.len())]);
    println!("⏱️  Time: {} ms\n", stats_fast.time_ms);

    // Calculate speedup
    println!("🚀 PERFORMANCE IMPROVEMENT");
    println!("{}", "=".repeat(50));
    if stats_fast.time_ms > 0 {
        let speedup = stats_slow.time_ms as f64 / stats_fast.time_ms as f64;
        println!("Speedup: {:.1}x faster!", speedup);
        
        if speedup >= 100.0 {
            println!("🥇 Status: RUST NINJA! (100x+ faster)");
        } else if speedup >= 50.0 {
            println!("🥈 Status: Excellent! (50x+ faster)");
        } else if speedup >= 10.0 {
            println!("🥉 Status: Good job! (10x+ faster)");
        } else {
            println!("📈 Status: Getting there... ({}x faster)", speedup as usize);
        }
    } else {
        println!("⚡ Too fast to measure accurately!");
    }

    println!("\n📝 KEY OPTIMIZATIONS APPLIED:");
    println!("  1. Single pass through text (was 4 separate passes)");
    println!("  2. Removed unnecessary .clone() calls");
    println!("  3. Used BinaryHeap for top-K (O(n log k) vs O(n²))");
    println!("  4. In-place character filtering without intermediate allocations");
    println!("  5. Efficient longest words using heap instead of full sort");
}