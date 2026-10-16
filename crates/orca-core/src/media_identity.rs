//! Shared conservative metadata matching for online media lookups.
pub(crate) fn names_match(query: &str, result: &str) -> bool {
    let normalize = |s: &str| {
        s.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
    };
    let query = normalize(query);
    !query.is_empty() && query == normalize(result)
}
pub(crate) fn album_matches(query: &str, result: &str) -> bool {
    fn album(s: &str) -> &str {
        let s = s.trim();
        for suffix in [" - Single", " - EP"] {
            if s.to_lowercase().ends_with(&suffix.to_lowercase()) {
                return s[..s.len() - suffix.len()].trim();
            }
        }
        s
    }
    let query = album(query);
    let result = album(result);
    if names_match(query, result) {
        return true;
    }

    // Providers may spell the same deluxe release with an expanded edition label
    // and a remaster year. Keep the deluxe marker on both sides so a standard
    // release or a compilation cannot satisfy a deluxe album lookup.
    fn deluxe_base(value: &str) -> Option<&str> {
        let mut value = value.trim();
        if let Some((base, bracket)) = value.rsplit_once(" [") {
            let bracket = bracket.strip_suffix(']')?.to_ascii_lowercase();
            let (year, remaster) = bracket.split_once(' ')?;
            if year.len() == 4
                && year.bytes().all(|c| c.is_ascii_digit())
                && matches!(remaster, "remaster" | "remastered")
            {
                value = base.trim();
            }
        }
        let lower = value.to_ascii_lowercase();
        for suffix in [" (deluxe edition)", " (deluxe)"] {
            if lower.ends_with(suffix) {
                return Some(value[..value.len() - suffix.len()].trim());
            }
        }
        None
    }

    match (deluxe_base(query), deluxe_base(result)) {
        (Some(query), Some(result)) => names_match(query, result),
        _ => false,
    }
}
