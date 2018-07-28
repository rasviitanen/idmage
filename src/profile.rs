pub struct Profile {
    pub name: String,
    pub slogan: String,
    pub font_family: Vec<String>,
    pub text_colors: Vec<String>,
    pub primary_colors: Vec<String>,
    pub background_colors: Vec<String>
}

impl Profile {
    pub fn new() -> Profile {
        let mut profile = Profile {
            name: "Picaas".into(),
            slogan: "Generated graphics".into(),
            font_family: Vec::new(),
            text_colors: Vec::new(),
            primary_colors: Vec::new(),
            background_colors: Vec::new(),
        };

        profile.text_colors.push("white".into());
        profile.text_colors.push("yellow".into());        
        profile.primary_colors.push("#0F0461".into());
        profile.primary_colors.push("#1E4FF2".into());
        profile.primary_colors.push("yellow".into());
        profile.background_colors.push("#04011A".into());
        profile.font_family.push("Zilla Slab".into());

        profile
    }
}