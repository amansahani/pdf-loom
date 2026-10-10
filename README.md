# pdf-loom

> **Blazing fast, 100% client-side PDF manipulation engine powered by Rust & WebAssembly.**

[![npm version](https://img.shields.io/npm/v/pdf-loom.svg)](https://www.npmjs.com/package/pdf-loom)
[![GitHub](https://img.shields.io/badge/GitHub-amansahani%2Fpdf--loom-blue?logo=github)](https://github.com/amansahani/pdf-loom)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Bundle Size](https://img.shields.io/badge/bundle%20size-~120%20kB%20gzipped-brightgreen)](#performance)

`pdf-loom` brings core PDF manipulation tasks (merge, split/extract, rotate, inspect, and sample generation) directly to the browser with zero server roundtrips, zero external native dependencies, and unmatched privacy.

- **NPM Package**: [https://www.npmjs.com/package/pdf-loom](https://www.npmjs.com/package/pdf-loom)
- **GitHub Repository**: [https://github.com/amansahani/pdf-loom](https://github.com/amansahani/pdf-loom)
- **Issue Tracker**: [https://github.com/amansahani/pdf-loom/issues](https://github.com/amansahani/pdf-loom/issues)

---

## ✨ Features

- 🔒 **100% Private & Client-Side**: No document bytes ever leave the client machine.
- ⚡ **High Performance**: Native WebAssembly core powered by Rust.
- 🪶 **Extremely Lightweight**: ~120 kB gzipped (~300 kB unpacked).
- 🧩 **Zero External Heavy Dependencies**: No headless Chrome, no Java, no C++ bindings required.
- 📄 **TypeScript Native**: Full typing support out of the box (`.d.ts`).
- 🌐 **Modern Framework Ready**: Works seamlessly in Next.js, Vite, React, Vue, Svelte, or vanilla JS.

---

## 📦 Installation

```bash
npm install pdf-loom
```
or with yarn / pnpm / bun:
```bash
yarn add pdf-loom
pnpm add pdf-loom
bun add pdf-loom
```

---

## 🚀 Quick Start

### 1. Merge Multiple PDFs

```typescript
import { merge_pdfs } from "pdf-loom";

// In browser: buffers are Uint8Array instances
const fileABytes = new Uint8Array(await fileA.arrayBuffer());
const fileBBytes = new Uint8Array(await fileB.arrayBuffer());

// Merge PDFs
const mergedPdfBytes = merge_pdfs([fileABytes, fileBBytes]);

// Download or display
const blob = new Blob([mergedPdfBytes], { type: "application/pdf" });
const downloadUrl = URL.createObjectURL(blob);
```

### 2. Extract / Split Pages

```typescript
import { extract_pages } from "pdf-loom";

const pdfBytes = new Uint8Array(await file.arrayBuffer());

// Extract page 1, 3, and 5 (1-based index)
const selectedPages = new Uint32Array([1, 3, 5]);
const extractedPdfBytes = extract_pages(pdfBytes, selectedPages);
```

### 3. Rotate Pages

```typescript
import { rotate_pages } from "pdf-loom";

const pdfBytes = new Uint8Array(await file.arrayBuffer());

// Rotate pages 1 and 2 clockwise by 90 degrees (degrees can be 90, 180, 270)
const pagesToRotate = new Uint32Array([1, 2]);
const rotatedPdfBytes = rotate_pages(pdfBytes, pagesToRotate, 90);
```

### 4. Compress PDF (Portal Size Limits)

Designed specifically for university, exam, and government application portals (e.g., UPSC, SSC, JEE, GATE, state admission boards requiring strict `<100KB` / `<200KB` PDF sizes):

```typescript
import { compress_pdf } from "pdf-loom";

const pdfBytes = new Uint8Array(await file.arrayBuffer());

// Level 1: Light (Lossless Deflate level 9 + dead object pruning)
// Level 2: Medium (Aggressive metadata, thumbnail, & piece info stripping + best compression)
// Level 3: Extreme (Portal compact: strips annotations, bookmarks, outlines, piece info, structural tree roots)
const compressedBytes = compress_pdf(pdfBytes, 3);

// Download compressed file
const blob = new Blob([compressedBytes], { type: "application/pdf" });
```

### 5. Inspect PDF Metadata

```typescript
import { get_pdf_info } from "pdf-loom";

const info = get_pdf_info(pdfBytes);
console.log(info);
// Output:
// {
//   page_count: 5,
//   version: "1.5",
//   is_encrypted: false
// }
```

### 6. Generate Test PDF

```typescript
import { create_sample_pdf } from "pdf-loom";

// Generate a sample 3-page PDF with custom title
const sampleBytes = create_sample_pdf(3, "Invoice Report #1042");
```

---

## ⚡ Integration with Next.js (App Router / Pages)

### 1. Configure Webpack in `next.config.mjs`

WebAssembly loading requires asynchronous WASM support in Next.js:

```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (config) => {
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
      layers: true,
    };
    return config;
  },
};

export default nextConfig;
```

### 2. Client-Side Dynamic Import Component

```tsx
"use client";

import React, { useState } from "react";

export default function PdfMergeComponent() {
  const [loading, setLoading] = useState(false);

  const handleMergeFiles = async (files: File[]) => {
    setLoading(true);
    try {
      // Dynamic import to prevent SSR execution
      const { merge_pdfs } = await import("pdf-loom");

      const buffers: Uint8Array[] = [];
      for (const file of files) {
        buffers.push(new Uint8Array(await file.arrayBuffer()));
      }

      const mergedBytes = merge_pdfs(buffers);

      // Download trigger
      const blob = new Blob([mergedBytes], { type: "application/pdf" });
      const link = document.createElement("a");
      link.href = URL.createObjectURL(blob);
      link.download = "merged-document.pdf";
      link.click();
    } catch (err) {
      console.error("Merge error:", err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <button disabled={loading} onClick={() => {/* pass files */}}>
        {loading ? "Merging in WebAssembly..." : "Merge PDFs"}
      </button>
    </div>
  );
}
```

---

## 📚 API Reference

### `merge_pdfs(pdf_buffers: Uint8Array[]): Uint8Array`
Combines multiple PDF documents in the order provided into a single PDF document.

- **Parameters**: `pdf_buffers`: Array of `Uint8Array` byte buffers.
- **Returns**: `Uint8Array` containing the combined PDF binary.

---

### `extract_pages(pdf_bytes: Uint8Array, page_numbers: Uint32Array): Uint8Array`
Extracts a subset of pages into a new document.

- **Parameters**:
  - `pdf_bytes`: Source PDF bytes.
  - `page_numbers`: `Uint32Array` containing 1-based page indices (e.g. `new Uint32Array([1, 4])`).
- **Returns**: `Uint8Array` containing the new extracted PDF.

---

### `rotate_pages(pdf_bytes: Uint8Array, page_numbers: Uint32Array, degrees: number): Uint8Array`
Rotates target pages by the specified angle.

- **Parameters**:
  - `pdf_bytes`: Source PDF bytes.
  - `page_numbers`: `Uint32Array` containing 1-based page indices.
  - `degrees`: `number` (typically `90`, `180`, or `270`).
- **Returns**: `Uint8Array` of the rotated PDF.

---

### `compress_pdf(pdf_bytes: Uint8Array, level: number): Uint8Array`
Recompresses streams and strips unreferenced metadata to minimize file size.

- **Parameters**:
  - `pdf_bytes`: Source PDF byte array.
  - `level`: `number`
    - `1`: **Standard/Light** - Flate2 Deflate Level 9 lossless compression and dead object pruning.
    - `2`: **High** - Prunes document metadata, piece info, structural tree roots, and thumbnails.
    - `3`: **Extreme / Portal Compact** - Tailored for Indian college/job portal restrictions; strips outlines, annotations, bookmarks, piece info, and applies maximum stream re-encoding.
- **Returns**: `Uint8Array` containing the compressed PDF binary.

---

### `get_pdf_info(pdf_bytes: Uint8Array): PdfInfo`
Parses and returns structural information about a document.

- **Returns**:
  ```typescript
  interface PdfInfo {
    page_count: number;
    version: string;
    is_encrypted: boolean;
  }
  ```

---

### `create_sample_pdf(page_count: number, title: string): Uint8Array`
Generates a valid, minimal blank template PDF with header text for testing or templating.

---

## 🛡️ License

MIT © [Aman Sahani](https://github.com/amansahani)
