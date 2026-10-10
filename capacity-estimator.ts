/**
 * Device Capability and Safe Capacity Estimator
 *
 * WebAssembly in 32-bit browsers has a physical ceiling of 4GB address space,
 * but browsers usually limit heap allocations to 1GB - 2GB before crashing.
 *
 * This module computes the lowest conservative maximum limit of documents/pages
 * that can be processed safely on the client device without running out of memory.
 */

export interface DeviceEstimate {
  deviceRamGB: number;
  cpuCores: number;
  safeMemoryBudgetMB: number;
  operation: 'merge' | 'split' | 'rotate' | 'compress';
  safeMaxDocuments: number;
  safeMaxTotalSizeMB: number;
  warningNotice: string;
}

/**
 * Estimates the lowest conservative maximum number of documents/payload
 * that the client browser can safely handle.
 */
export function estimateSafeCapacity(
  operation: 'merge' | 'split' | 'rotate' | 'compress' = 'merge',
  avgDocSizeMB: number = 3.0
): DeviceEstimate {
  // 1. Detect hardware metrics with safe fallbacks
  // navigator.deviceMemory returns 0.5, 1, 2, 4, 8 (in GiB).
  // Defaults to 2GB if unavailable (e.g., Firefox/Safari or privacy shields)
  const nav = typeof navigator !== 'undefined' ? navigator : null;
  const reportedRam = (nav as any)?.deviceMemory ? (nav as any).deviceMemory : 2;
  const cpuCores = nav?.hardwareConcurrency ? nav.hardwareConcurrency : 2;

  // 2. Compute safe WASM memory budget
  // Never exceed 25% of device RAM or 1000MB (WASM 32-bit safe envelope)
  const safeMemoryBudgetMB = Math.min(reportedRam * 1024 * 0.25, 1000);

  // 3. Operational memory multiplication factor
  // Parsing and reconstructing PDF object trees in memory causes temporary spikes:
  // - rotate/reorder: ~2.5x overhead
  // - merge/split: ~4x overhead
  // - compress/re-encode: ~6x overhead
  let memoryOverheadMultiplier = 4.0;
  switch (operation) {
    case 'rotate':
      memoryOverheadMultiplier = 2.5;
      break;
    case 'split':
      memoryOverheadMultiplier = 3.0;
      break;
    case 'merge':
      memoryOverheadMultiplier = 4.0;
      break;
    case 'compress':
      memoryOverheadMultiplier = 6.0;
      break;
  }

  // 4. Safe maximum total input batch size (MB)
  // Apply a 0.65 conservative safety buffer factor to always estimate lowest safe maximum
  const rawBatchSizeMB = safeMemoryBudgetMB / memoryOverheadMultiplier;
  const safeMaxTotalSizeMB = Math.max(5, Math.floor(rawBatchSizeMB * 0.65));

  // 5. Lowest maximum document count
  const safeMaxDocuments = Math.max(1, Math.floor(safeMaxTotalSizeMB / avgDocSizeMB));

  let warningNotice = `Based on your device (~${reportedRam}GB detected RAM), safe limit is ${safeMaxDocuments} files (~${safeMaxTotalSizeMB}MB total).`;
  if (reportedRam <= 2) {
    warningNotice += ' Low-memory environment detected; batch sizes are restricted to prevent browser tab crashes.';
  }

  return {
    deviceRamGB: reportedRam,
    cpuCores,
    safeMemoryBudgetMB: Math.round(safeMemoryBudgetMB),
    operation,
    safeMaxDocuments,
    safeMaxTotalSizeMB,
    warningNotice
  };
}
