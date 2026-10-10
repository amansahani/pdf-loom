/**
 * Node.js Test Suite for pdf-loom WASM module
 */
const { 
  create_sample_pdf, 
  get_pdf_info, 
  merge_pdfs, 
  extract_pages, 
  rotate_pages 
} = require('./pkg-node/pdf_loom.js');
const assert = require('assert');

console.log('🧪 Starting Node.js WASM Module Tests...\n');

try {
  // Test 1: create_sample_pdf & get_pdf_info
  console.log('Test 1: create_sample_pdf and get_pdf_info');
  const pdfA = create_sample_pdf(3, 'Document A');
  assert(pdfA instanceof Uint8Array, 'Generated PDF should be Uint8Array');
  assert(pdfA.length > 500, 'PDF bytes should be non-trivial');

  const infoA = get_pdf_info(pdfA);
  console.log('  PDF A info:', infoA);
  assert.strictEqual(infoA.page_count, 3, 'Page count should be 3');
  assert.strictEqual(infoA.is_encrypted, false, 'Should not be encrypted');
  console.log('  ✅ Test 1 Passed!\n');

  // Test 2: merge_pdfs
  console.log('Test 2: merge_pdfs');
  const pdfB = create_sample_pdf(2, 'Document B');
  const infoB = get_pdf_info(pdfB);
  assert.strictEqual(infoB.page_count, 2, 'Doc B page count should be 2');

  const merged = merge_pdfs([pdfA, pdfB]);
  assert(merged instanceof Uint8Array, 'Merged output should be Uint8Array');
  const mergedInfo = get_pdf_info(merged);
  console.log('  Merged PDF info:', mergedInfo);
  assert.strictEqual(mergedInfo.page_count, 5, 'Merged page count should be 3 + 2 = 5');
  console.log('  ✅ Test 2 Passed!\n');

  // Test 3: extract_pages (Split)
  console.log('Test 3: extract_pages (Split 1st and 3rd pages from Doc A)');
  const extracted = extract_pages(pdfA, new Uint32Array([1, 3]));
  const extractedInfo = get_pdf_info(extracted);
  console.log('  Extracted PDF info:', extractedInfo);
  assert.strictEqual(extractedInfo.page_count, 2, 'Extracted page count should be 2');
  console.log('  ✅ Test 3 Passed!\n');

  // Test 4: rotate_pages
  console.log('Test 4: rotate_pages');
  const rotated = rotate_pages(pdfA, new Uint32Array([1, 2]), 90);
  const rotatedInfo = get_pdf_info(rotated);
  assert.strictEqual(rotatedInfo.page_count, 3, 'Page count should remain 3 after rotation');
  console.log('  ✅ Test 4 Passed!\n');

  console.log('🎉 ALL Node.js WASM TESTS PASSED SUCCESSFULLY! 🚀');
} catch (error) {
  console.error('❌ Test failed with error:', error);
  process.exit(1);
}
