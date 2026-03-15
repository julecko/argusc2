# SvelteTemplate

A minimal **SvelteKit** template project using **Vite**, **SCSS**, **TypeScript**, **Prettier**, and **ESLint**.

---

## Features

* **SvelteKit**: Framework for building modern web apps.
* **Vite**: Fast bundler & dev server.
* **TypeScript**: Type safety for JS/TS code.
* **SCSS**: Flexible styling with variables and mixins.
* **Prettier**: Automatic code formatting.
* **ESLint**: Code linting for best practices.
* **Path aliases**: `$lib`, `@components`, `@styles`, `@utils` for clean imports.

---

## Project Structure

```
src/
 ├─ lib/
 │   ├─ components/       # Reusable Svelte components
 │   ├─ styles/           # Global SCSS, variables, mixins
 │   ├─ utils/            # Helper functions
 │   └─ assets/           # Images, icons, etc.
 ├─ routes/               # SvelteKit pages and layouts
 └─ app.html              # HTML template
```

---

## Setup

1. Install dependencies:

```bash
npm install
```

2. Run dev server:

```bash
npm run dev
```

3. Build for production:

```bash
npm run build
```

4. Preview production build:

```bash
npm run preview
```

---

## Path Aliases

Configured in `svelte.config.js` and `tsconfig.json`:

| Alias         | Path                 |
| ------------- | -------------------- |
| `$lib`        | `src/lib`            |
| `@components` | `src/lib/components` |
| `@styles`     | `src/lib/styles`     |
| `@utils`      | `src/lib/utils`      |

---

## Code Quality

* **Prettier**: Auto-formats code. Run with:

```bash
npx prettier --write .
```

* **ESLint**: Lints code. Run with:

```bash
npx eslint src --fix
```

---

## TypeScript Config

* Extends SvelteKit’s auto-generated TS config.
* Path aliases defined for `$lib`, `@components`, `@styles`, `@utils`.
