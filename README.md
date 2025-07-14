# NeonPack

A fast global JavaScript package manager and runtime engine built in Rust.  
Install NPM packages globally, rewrite imports smartly, and run your JavaScript project with zero `node_modules` required.

---

## Key Features

- Global package installation to `~/.neonpack/lib`
- Auto-import rewriting for bare modules (e.g., `import lodash from "lodash"`)
- Fully integrates with your project’s `package.json`
- No `node_modules/`, no symlinks, no hard links
- Designed for speed and portability

---

## Installation

```bash
git clone https://github.com/yourusername/neonpack
cd neonpack
./build.sh
```

---

## CLI Commands

### `neonpack install <package>`

Install an NPM package globally and update your project’s `package.json`.

```bash
neonpack install lodash
```

Outcome:
- Downloads and extracts to: `~/.neonpack/lib/lodash/<version>/`
- Updates your local `package.json` dependencies

---

### `neonpack install -g <package>`

Install a package globally without modifying the current project.

```bash
neonpack install -g lodash
```

Outcome:
- Downloads and extracts to: `~/.neonpack/lib/lodash/<version>/`
- Does not touch `package.json`

---

### `neonpack use <package>`

Activate a globally installed package by adding it to your current project's `package.json`.

```bash
neonpack use lodash
```

Outcome:
- Looks for the package in global storage
- Adds it to your project’s `package.json` with the correct version

---

### `neonpack run <script>`

Run a script from your `package.json` with automatic import rewriting.

```bash
neonpack run dev
```

If your `package.json` contains:

```json
{
  "scripts": {
    "dev": "node src/index.js"
  }
}
```

Then `neonpack run dev` will:
- Rewrite ESModule imports like `import x from "lodash"` into full file:// paths
- Run the transformed file using Node.js

---

### `neonpack remove <package>`

Remove a package from your project’s `package.json`.

```bash
neonpack remove lodash
```

Outcome:
- Deletes entry from `dependencies` or `devDependencies` in `package.json`

---

### `neonpack remove -g <package>`

Remove a package from global storage.

```bash
neonpack remove -g lodash
```

Outcome:
- Deletes `~/.neonpack/lib/lodash/`

---

## How It Works

- `install` fetches the package tarball from the npm registry
- `extract_tarball` unpacks it to `~/.neonpack/lib/<pkg>/<version>`
- `run` rewrites ESM imports to absolute `file://` URLs
- Packages are never copied into project folders
- No `node_modules`, no symbolic links, no hard links

---

## Example Structure

```
~/.neonpack/lib/
├── lodash/
│   └── 4.17.21/
│       ├── package.json
│       ├── lodash.js
│       └── ...
```
---
