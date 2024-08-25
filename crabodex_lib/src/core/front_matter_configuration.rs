use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

pub const DEFAULT_FRONT_MATTER_PREFIX: &str = "---";

pub struct FrontMatterConfig {
    pub prefix: String,
    pub suffix: String,
}

impl Default for FrontMatterConfig {
    fn default() -> Self {
        FrontMatterConfig {
            prefix: DEFAULT_FRONT_MATTER_PREFIX.to_string(),
            suffix: DEFAULT_FRONT_MATTER_PREFIX.to_string(),
        }
    }
}

pub static FRONT_MATTER_CONFIG: Lazy<Mutex<FrontMatterConfig>> = Lazy::new(|| {
    Mutex::new(FrontMatterConfig::default())
});

pub fn set_front_matter_config(prefix: &str, suffix: &str) {
    let mut config: MutexGuard<FrontMatterConfig> = FRONT_MATTER_CONFIG.lock().unwrap();
    config.prefix = prefix.to_string();
    config.suffix = suffix.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config: MutexGuard<FrontMatterConfig> = FRONT_MATTER_CONFIG.lock().unwrap();
        assert_eq!(config.prefix, DEFAULT_FRONT_MATTER_PREFIX);
        assert_eq!(config.suffix, DEFAULT_FRONT_MATTER_PREFIX);
    }

    #[test]
    fn test_multiple_changes() {
        let new_prefix: &str = "<<<";
        let new_suffix: &str = ">>>";
        set_front_matter_config(new_prefix, new_suffix);
        {
            let config: MutexGuard<FrontMatterConfig> = FRONT_MATTER_CONFIG.lock().unwrap();
            assert_eq!(config.prefix, new_prefix);
            assert_eq!(config.suffix, new_suffix);
        }

        let new_prefix: &str = "(((";
        let new_suffix: &str = ")))";
        set_front_matter_config(new_prefix, new_suffix);
        {
            let config: MutexGuard<FrontMatterConfig> = FRONT_MATTER_CONFIG.lock().unwrap();
            assert_eq!(config.prefix, new_prefix);
            assert_eq!(config.suffix, new_suffix);
        }

        // Reset to default for other tests
        set_front_matter_config(DEFAULT_FRONT_MATTER_PREFIX, DEFAULT_FRONT_MATTER_PREFIX);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let thread_count: i32 = 10;
        let threads: Vec<_> = (0..thread_count)
            .map(|i| {
                thread::spawn(move || {
                    let prefix = format!("prefix{}", i);
                    let suffix = format!("suffix{}", i);
                    set_front_matter_config(&prefix, &suffix);

                    // Small delay to increase chances of interleaving
                    thread::sleep(std::time::Duration::from_millis(10));

                    let config: MutexGuard<FrontMatterConfig> = FRONT_MATTER_CONFIG.lock().unwrap();
                    (config.prefix.clone(), config.suffix.clone())
                })
            })
            .collect();

        let results: Vec<_> = threads
            .into_iter()
            .map(|t| t.join().unwrap())
            .collect();

        // Check that all results are valid (i.e., matching prefix and suffix numbers)
        for (prefix, suffix) in results {
            assert!(prefix.starts_with("prefix"));
            assert!(suffix.starts_with("suffix"));
            let prefix_num = prefix.trim_start_matches("prefix");
            let suffix_num = suffix.trim_start_matches("suffix");
            assert_eq!(prefix_num, suffix_num);
        }

        // Reset to default for other tests
        set_front_matter_config(DEFAULT_FRONT_MATTER_PREFIX, DEFAULT_FRONT_MATTER_PREFIX);
    }
}
