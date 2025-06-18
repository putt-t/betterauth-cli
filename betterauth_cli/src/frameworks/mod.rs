pub mod nextjs;

#[derive(Copy, Clone, PartialEq)]
pub enum Framework {
    Astro,
    Remix,
    Next,
    Nuxt,
    SvelteKit,
    SolidStart,
    TanstackStart,
}

impl std::fmt::Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Framework::Astro => write!(f, "Astro"),
            Framework::Remix => write!(f, "Remix"),
            Framework::Next => write!(f, "Next"),
            Framework::Nuxt => write!(f, "Nuxt"),
            Framework::SvelteKit => write!(f, "SvelteKit"),
            Framework::SolidStart => write!(f, "SolidStart"),
            Framework::TanstackStart => write!(f, "Tanstack Start"),
        }
    }
}
