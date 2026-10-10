---
name: pdf-loom
description: Harness for integrating and using pdf-loom, an in-browser WebAssembly PDF manipulation tool (merge, split/extract, rotate, inspect) in web applications.
---

# `pdf-loom` Harness Skill

`pdf-loom` brings 100% client-side PDF manipulation directly into the browser with zero server roundtrips, powered by Rust & WebAssembly.

- **NPM Package**: [`pdf-loom`](https://www.npmjs.com/package/pdf-loom)
- **GitHub Repository**: [https://github.com/amansahani/pdf-loom](https://github.com/amansahani/pdf-loom)
- **Bundle Size**: ~120 kB gzipped (~300 kB uncompressed)

---

## 1. Installation

```bash
npm install pdf-loom
```

---

## 2. Next.js Configuration (`next.config.mjs`)

Enable asynchronous WebAssembly in Next.js Webpack configuration:

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

---

## 3. Harness Integration Patterns

> [!IMPORTANT]
> Because WebAssembly interacts with browser buffers and memory, do not execute `pdf-loom` on the server during SSR. Always invoke it within `"use client"` components, dynamically imported or inside client event handlers.

### A. Merging Multiple PDFs

```typescript
"use client";

import { useState } from "react";

export function usePdfMerger() {
  const [isProcessing, setIsProcessing] = useState(false);

  const mergeFiles = async (files: File[]): Promise<string> => {
    setIsProcessing(true);
    try {
      const { merge_pdfs } = await import("pdf-loom");

      // 1. Read files into Uint8Array buffers
      const buffers: Uint8Array[] = [];
      for (const file of files) {
        const arrayBuffer = await file.arrayBuffer();
        buffers.push(new Uint8Array(arrayBuffer));
      }

      // 2. Perform client-side WASM merge
      const mergedBytes = merge_pdfs(buffers);

      // 3. Return object URL for download or preview
      const blob = new Blob([mergedBytes], { type: "application/pdf" });
      return URL.createObjectURL(blob);
    } finally {
      setIsProcessing(false);
    }
  };

  return { mergeFiles, isProcessing };
}
```

---

### B. Extract / Split Pages

```typescript
import { extract_pages } from "pdf-loom";

// Extract pages 1, 3, and 5 (1-based index)
async function splitPdf(file: File, pageNumbers: number[]) {
  const bytes = new Uint8Array(await file.arrayBuffer());
  const selectedPages = new Uint32Array(pageNumbers);

  const extractedBytes = extract_pages(bytes, selectedPages);
  return new Blob([extractedBytes], { type: "application/pdf" });
}
```

---

### C. Rotate Pages

```typescript
import { rotate_pages } from "pdf-loom";

// Rotate target pages clockwise by 90 degrees (degrees: 90, 180, 270)
async function rotatePdfPages(file: File, pageNumbers: number[], degrees: number) {
  const bytes = new Uint8Array(await file.arrayBuffer());
  const targetPages = new Uint32Array(pageNumbers);

  const rotatedBytes = rotate_pages(bytes, targetPages, degrees);
  return new Blob([rotatedBytes], { type: "application/pdf" });
}
```

---

### D. Compress PDF (Strict Portal Limits)

```typescript
import { compress_pdf } from "pdf-loom";

// Compress PDF with levels: 1 = Light, 2 = Medium, 3 = Extreme (Portal compact)
async function compressDocument(file: File, level: number = 3) {
  const bytes = new Uint8Array(await file.arrayBuffer());
  const compressedBytes = compress_pdf(bytes, level);
  return new Blob([compressedBytes], { type: "application/pdf" });
}
```

---

### E. Read PDF Metadata (Page Count, Encryption, Version)

```typescript
import { get_pdf_info } from "pdf-loom";

async function inspectPdf(file: File) {
  const bytes = new Uint8Array(await file.arrayBuffer());
  const info = get_pdf_info(bytes);
  // Returns: { page_count: number, version: string, is_encrypted: boolean }
  return info;
}
```

---

### F. Generate Sample / Template PDF

```typescript
import { create_sample_pdf } from "pdf-loom";

function generateTestDocument(pageCount: number, title: string) {
  const sampleBytes = create_sample_pdf(pageCount, title);
  return new Blob([sampleBytes], { type: "application/pdf" });
}
```

---

## 4. API Reference Summary

| Function | Type Signature | Description |
| :--- | :--- | :--- |
| `merge_pdfs` | `(pdf_buffers: Uint8Array[]) => Uint8Array` | Merges multiple PDF byte arrays in sequential order. |
| `extract_pages` | `(pdf_bytes: Uint8Array, page_numbers: Uint32Array) => Uint8Array` | Extracts selected 1-based page indices into a new PDF. |
| `rotate_pages` | `(pdf_bytes: Uint8Array, page_numbers: Uint32Array, degrees: number) => Uint8Array` | Rotates specified pages clockwise (90, 180, 270). |
| `compress_pdf` | `(pdf_bytes: Uint8Array, level: number) => Uint8Array` | Recompresses streams & strips non-essential metadata (Level 1, 2, 3). |
| `get_pdf_info` | `(pdf_bytes: Uint8Array) => { page_count, version, is_encrypted }` | Inspects document header, page count, and encryption state. |
| `create_sample_pdf` | `(page_count: number, title: string) => Uint8Array` | Creates a valid minimalist test PDF with title and page numbers. |
