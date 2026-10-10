# pdf-loom

> **Blazing fast, 100% client-side PDF manipulation engine powered by Rust & WebAssembly.**

[![npm version](https://img.shields.io/npm/v/pdf-loom.svg)](https://www.npmjs.com/package/pdf-loom)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Bundle Size](https://img.shields.io/badge/bundle%20size-~120%20kB%20gzipped-brightgreen)](#performance)

`pdf-loom` brings core PDF manipulation tasks (merge, split/extract, rotate, inspect, and sample generation) directly to the browser with zero server roundtrips, zero external native dependencies, and unmatched privacy.

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

### 4. Inspect PDF Metadata

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

### 5. Generate Test PDF

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
