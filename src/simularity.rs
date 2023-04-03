
pub fn is_related(input_string: &str, keywords: &[&str]) -> bool {
    let split_keywords = ['-', ' ', ':', '@', '_', '.', ',', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let input_words = to_lowercase(input_string.split(&split_keywords[..]).filter(|s| s.len() > 1));
    let keywords_lowercase = to_lowercase(keywords.iter().cloned());

    input_words.iter().any(|word| {
        keywords_lowercase.iter().any(|keyword| {
            let max_edit_distance = (keyword.len() / 3).max(1);
            let edit_distance = levenshtein_distance(word, keyword);
            edit_distance <= max_edit_distance
        })
    })
}

fn to_lowercase<'a, I>(words: I) -> Vec<String>
where
    I: Iterator<Item = &'a str>,
{
    words.map(|word| word.to_lowercase()).collect()
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();
    let mut matrix = vec![vec![0; s2_len + 1]; s1_len + 1];

    for i in 0..=s1_len {
        matrix[i][0] = i;
    }

    for j in 0..=s2_len {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let substitution_cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                matrix[i][j + 1] + 1,
                std::cmp::min(matrix[i + 1][j] + 1, matrix[i][j] + substitution_cost),
            );
        }
    }

    matrix[s1_len][s2_len]
}
