# BETTER-AUTH CLI

A command-line interface tool that automatically generates and configures [BETTER-AUTH](https://www.better-auth.com/) projects based on your framework, database, and social sign-on preferences.

yes i am making both the typescript and rust version at the same time.

## Disclaimer

I just started re-learning Rust so this is all relatively new to me. Feel free to bash my code and give me feedback on how to optimize / improve. also might have some AI slop in there, sometimes be using tab in cursor to help me out as well.

## 🚀 Features

### Current Features
- **Framework Selection**: Choose from popular web frameworks
  - ✅ Next.js (implementing)
  - 🚧 Astro (planned)
  - 🚧 Remix (planned)
  - 🚧 Nuxt (planned)
  - 🚧 SvelteKit (planned)
  - 🚧 SolidStart (planned)
  - 🚧 Tanstack Start (planned)

- **Automated Project Setup** (Next.js):
  - Creates new project using `create-next-app`
  - Installs BetterAuth dependency
  - Generates secure random `BETTER_AUTH_SECRET`
  - Creates `.env.local` with necessary environment variables
  - Sets up `auth.ts` configuration file in your preferred location

### Planned Features
- **Database Integration**: Automatic database setup and configuration
  - PostgreSQL
  - MySQL
  - SQLite
  - MS SQL

- **Social Sign-On Configuration**: Pre-configured social providers
  - Google
  - GitHub
  - Discord
  - Facebook
  - Twitter/X
  - And more...

- **Automatic Backend Setup**

## 📦 Installation

### Prerequisites
- [Rust](https://rustup.rs/) (for building from source)
- [Node.js](https://nodejs.org/) (for framework project creation)
- [npm](https://www.npmjs.com/) or [yarn](https://yarnpkg.com/) or [pnpm](https://pnpm.io/)

### Build from Source
```bash
git clone https://github.com/putt-t/betterauth-cli.git
cd betterauth-cli/betterauth_cli
cargo build --release
```

## 🛠️ Usage

Run the CLI tool and follow the interactive prompts:

```bash
./target/release/betterauth_cli
```

### Current Workflow (Next.js)
1. **Framework Selection**: Choose your preferred framework
2. **Project Name**: Enter your project name
3. **Auth File Location**: Choose where to place the `auth.ts` file:
   - Project root
   - `lib/` directory
   - `utils/` directory

The CLI will:
- Create a new Next.js project
- Install BETTER-AUTH
- Generate a secure authentication secret
- Set up environment variables
- Create the initial auth configuration

## 📁 Generated Project Structure

For a Next.js project, the CLI generates:

```
your-project/
├── .env.local                 # Environment variables with BETTER_AUTH_SECRET
├── auth.ts                    # BetterAuth configuration (location varies)
├── package.json              # Updated with better-auth dependency
└── ... (standard Next.js files)
```

## 🔧 Configuration

The generated `.env.local` includes:
- `BETTER_AUTH_SECRET`: Cryptographically secure random secret
- `BETTER_AUTH_URL`: Base URL for your application

The `auth.ts` file provides a starting point for BetterAuth configuration:
```typescript
import { betterAuth } from "better-auth";

export const auth = betterAuth({
    //...
});
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the terms specified in the [LICENSE](LICENSE) file.

## 🔗 Related

- [BETTER-AUTH Documentation](https://www.better-auth.com/docs)
- [BETTER-AUTH GitHub Repository](https://github.com/better-auth/better-auth)
