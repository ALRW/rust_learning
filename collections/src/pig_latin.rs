pub fn to_pig_latin(s: String) -> String {
    let mut ps = s.chars().peekable();
    let mut ns = String::new();
    while let Some(c) = ps.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                ns.push(c);
                String::from("-ay")
            }
            'a'..='z' | 'A'..='Z' => {
                format!("-{}ay", c)
            }
            _ => {
                ns.push(c);
                continue;
            }
        };

        while let Some(&c) = ps.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    ps.next();
                    ns.push(c);
                }
                _ => {
                    break;
                }
            }
        }
        ns.push_str(&suffix);
    }
    ns
}
