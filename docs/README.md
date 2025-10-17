# Project Documentation

This folder contains all technical documentation for the project.

## Available Documentation

- **[GETTING_STARTED.md](GETTING_STARTED.md)** - Step-by-step guide to start using this template
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Detailed architecture documentation
- **[TEMPLATE.md](TEMPLATE.md)** - Documentation template for new features

## Documentation Conventions

### File Naming

- All technical documentation files must be placed in the `docs/` folder
- Use descriptive uppercase names with underscores (e.g., `FEATURE_GUIDE.md`)
- Each significant feature, migration, or architectural decision should have its own `.md` file
- Include date and version information in documentation when relevant

### Documentation Types

- **Migration guides**: Follow pattern `{FEATURE}_MIGRATION.md`
- **Architecture decision records**: Follow pattern `ADR_{NUMBER}_{TITLE}.md`
- **How-to guides**: Follow pattern `HOWTO_{TOPIC}.md`
- **Feature documentation**: Follow pattern `{FEATURE}_GUIDE.md`

## Writing New Documentation

When adding new documentation:

1. Copy `TEMPLATE.md` to create a new file
2. Follow the naming conventions above
3. Include the following sections:
   - **Overview** - What is this about?
   - **Context** - Why is this important?
   - **Details** - How does it work?
   - **Examples** - Practical usage examples
   - **References** - Links to related documentation

## Maintaining Documentation

- Keep documentation up to date with code changes
- Add new documentation for new features
- Archive outdated documentation (move to `docs/archive/`)
- Review documentation quarterly

## Getting Help

If you have questions:
1. Check `GETTING_STARTED.md` first
2. Review `ARCHITECTURE.md` for design patterns
3. Look at code examples in `src/`
4. Check `projectmap.yaml` for project structure
