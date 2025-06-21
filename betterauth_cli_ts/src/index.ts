import { checkbox, select } from "@inquirer/prompts";

const framework = await select({
    message: "Choose your framework",
    choices: [
        { name: "Astro", value: "astro" },
        { name: "Remix", value: "remix" },
        { name: "Nextjs", value: "nextjs" },
        { name: "Nuxt", value: "nuxt" },  
        { name: "SvelteKit", value: "sveltekit" },
        { name: "SolidStrat", value: "solidstart" },
        { name: "Tanstack Start", value: "tanstackstart" },
    ],
});
