const { create_sample_pdf, merge_pdfs } = require('./pkg-node/pdf_loom.js');
const lopdf = require('fs');

const a = create_sample_pdf(2, "A");
const b = create_sample_pdf(1, "B");
const merged = merge_pdfs([a, b]);

console.log("Merged bytes length:", merged.length);
console.log("Merged text:", Buffer.from(merged).toString('latin1').slice(0, 1000));
console.log("Merged trailer:", Buffer.from(merged).toString('latin1').slice(-400));
