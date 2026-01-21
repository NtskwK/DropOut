# DropOut Documentation

This is the official documentation site for DropOut Minecraft Launcher, built with [Fumadocs](https://fumadocs.dev) and React Router v7.

## Overview

The documentation covers:
- **Getting Started**: Installation and first-time setup
- **Features**: Detailed guides for all launcher features
- **Architecture**: Technical design and implementation details
- **Development**: Building and contributing to DropOut
- **Troubleshooting**: Common issues and solutions

## Development

### Prerequisites

- Node.js 22+
- pnpm 9+

### Setup

Install dependencies:

```bash
pnpm install
```

### Run Development Server

```bash
pnpm dev
```

This starts the development server at `http://localhost:5173` with hot reload enabled.

### Build for Production

```bash
pnpm build
```

The production build will be output to the `build/` directory.

### Type Checking

```bash
pnpm types:check
```

### Linting and Formatting

```bash
# Check code
pnpm lint

# Format code
pnpm format
```

## Project Structure

```
packages/docs/
├── content/
│   └── docs/              # Documentation content (MDX)
│       ├── index.mdx      # Home page
│       ├── getting-started.mdx
│       ├── architecture.mdx
│       ├── development.mdx
│       ├── troubleshooting.mdx
│       └── features/      # Feature-specific docs
├── app/                   # React Router app
├── public/                # Static assets
├── source.config.ts       # Fumadocs configuration
└── react-router.config.ts # React Router configuration
```

## Writing Documentation

### MDX Format

All documentation is written in MDX (Markdown with JSX):

```mdx
---
title: Page Title
description: Page description for SEO
---

# Page Title

Content goes here...

<Cards>
  <Card title="Link Title" href="/path" />
</Cards>
```

### Available Components

Fumadocs provides several components:

- `<Card>` - Link cards
- `<Cards>` - Card container
- `<Callout>` - Info/warning boxes
- `<Tabs>` - Tabbed content
- `<Steps>` - Numbered steps
- Code blocks with syntax highlighting

### Adding New Pages

1. Create new `.mdx` file in `content/docs/`
2. Add frontmatter with title and description
3. Write content using MDX
4. Update `meta.json` to include the page
5. Test locally with `pnpm dev`

### Organizing Content

Use `meta.json` files to organize navigation:

```json
{
  "title": "Section Title",
  "pages": [
    "page1",
    "page2",
    {
      "title": "Subsection",
      "pages": ["sub1", "sub2"]
    }
  ]
}
```

## Deployment

The documentation is automatically deployed when changes are merged to the main branch.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally
5. Submit a pull request

## Links

- [DropOut Repository](https://github.com/HydroRoll-Team/DropOut)
- [Fumadocs](https://fumadocs.dev)
- [React Router](https://reactrouter.com)

## License

MIT License - see the main repository for details.
