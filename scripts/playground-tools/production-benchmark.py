#!/usr/bin/env python3
"""
DX Package Manager v3.0 - Production Benchmark Suite
Tests both cold start (3x target) and warm start (50x target)
"""

import subprocess
import time
import shutil
import os
import sys
from pathlib import Path
from datetime import datetime

# Colors
RED = '\033[0;31m'
GREEN = '\033[0;32m'
YELLOW = '\033[1;33m'
BLUE = '\033[0;34m'
CYAN = '\033[0;36m'
NC = '\033[0m'  # No Color

SCRIPT_DIR = Path(__file__).parent
TEST_DIR = SCRIPT_DIR / "real-world-test"
DX_BIN = Path("F:/Code/dx/crates/dx-js-package-manager/target/release/dx.exe")

def print_header():
    print("╔════════════════════════════════════════════════════════════╗")
    print("║   DX Package Manager v3.0 - Production Benchmark Suite    ║")
    print("╠════════════════════════════════════════════════════════════╣")
    print("║                                                            ║")
    print("║  Target 1: 3x faster than Bun (Cold Start)                ║")
    print("║  Target 2: 50x faster than Bun (Warm Start)               ║")
    print("║                                                            ║")
    print("╚════════════════════════════════════════════════════════════╝")
    print()

def run_command(cmd, cwd=None):
    """Run command and return execution time"""
    if cwd:
        cwd = str(cwd)
    start = time.time()
    result = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)
    elapsed = time.time() - start
    return elapsed, result

def clean_cache(full=False):
    """Clean node_modules and optionally all caches"""
    cwd = TEST_DIR
    if (cwd / "node_modules").exists():
        shutil.rmtree(cwd / "node_modules")
    if (cwd / "bun.lockb").exists():
        os.remove(cwd / "bun.lockb")
    if (cwd / "dx-lock.json").exists():
        os.remove(cwd / "dx-lock.json")
    if full:
        dx_cache = Path.home() / ".dx"
        if dx_cache.exists():
            shutil.rmtree(dx_cache)

def main():
    print_header()
    
    os.chdir(TEST_DIR)
    
    # Check DX binary
    if not DX_BIN.exists():
        print(f"{RED}Error: DX binary not found at {DX_BIN}{NC}")
        return 1
    
    print(f"{CYAN}Using DX binary: {DX_BIN}{NC}")
    print()
    
    # ═══════════════════════════════════════════════════════════════
    # PHASE 1: Cold Start Benchmark (3x target)
    # ═══════════════════════════════════════════════════════════════
    
    print("═══════════════════════════════════════════════════════════════")
    print("  PHASE 1: Cold Start Benchmark (3x Faster Target)")
    print("═══════════════════════════════════════════════════════════════")
    print()
    
    # Clean all caches
    print(f"{YELLOW}Cleaning all caches...{NC}")
    clean_cache(full=True)
    print("✓ All caches cleaned")
    print()
    
    # Bun baseline (cold)
    print(f"{BLUE}━━━ Bun Cold Install (Baseline) ━━━{NC}")
    bun_times = []
    for i in range(3):
        print(f"Installing with Bun (run {i+1}/3)... ", end='', flush=True)
        elapsed, _ = run_command("bun install", cwd=TEST_DIR)
        bun_times.append(elapsed)
        print(f"✓ {elapsed:.3f}s")
        clean_cache(full=True)
    
    bun_cold_avg = sum(bun_times) / len(bun_times)
    print()
    print(f"{BLUE}Bun Cold Average: {bun_cold_avg:.3f}s{NC}")
    print()
    
    # DX v3 cold start
    clean_cache(full=True)
    print(f"{GREEN}━━━ DX v3.0 Cold Install ━━━{NC}")
    dx_cold_times = []
    for i in range(3):
        print(f"Installing with DX v3 (run {i+1}/3)... ", end='', flush=True)
        elapsed, result = run_command(f'"{DX_BIN}" install --v3', cwd=TEST_DIR)
        dx_cold_times.append(elapsed)
        print(f"✓ {elapsed:.3f}s")
        clean_cache(full=True)
    
    dx_cold_avg = sum(dx_cold_times) / len(dx_cold_times)
    print()
    print(f"{GREEN}DX Cold Average: {dx_cold_avg:.3f}s{NC}")
    print()
    
    # Calculate cold speedup
    cold_speedup = bun_cold_avg / dx_cold_avg if dx_cold_avg > 0 else 0
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print(f"{CYAN}Cold Start Result: {cold_speedup:.2f}x faster than Bun{NC}")
    if cold_speedup >= 3.0:
        print(f"{GREEN}✅ COLD START TARGET ACHIEVED! (≥3x){NC}")
    else:
        print(f"{YELLOW}⚠️  Cold start: {cold_speedup:.2f}x (target: 3x){NC}")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print()
    
    # ═══════════════════════════════════════════════════════════════
    # PHASE 2: Warm Start Benchmark (50x target)
    # ═══════════════════════════════════════════════════════════════
    
    print("═══════════════════════════════════════════════════════════════")
    print("  PHASE 2: Warm Start Benchmark (50x Faster Target)")
    print("═══════════════════════════════════════════════════════════════")
    print()
    
    # Bun warm (with cache)
    print(f"{BLUE}━━━ Bun Warm Install (With Cache) ━━━{NC}")
    bun_warm_times = []
    for i in range(3):
        clean_cache(full=False)  # Keep cache, only remove node_modules
        print(f"Installing with Bun (run {i+1}/3)... ", end='', flush=True)
        elapsed, _ = run_command("bun install", cwd=TEST_DIR)
        bun_warm_times.append(elapsed)
        print(f"✓ {elapsed:.3f}s")
    
    bun_warm_avg = sum(bun_warm_times) / len(bun_warm_times)
    print()
    print(f"{BLUE}Bun Warm Average: {bun_warm_avg:.3f}s{NC}")
    print()
    
    # DX v3 warm
    print(f"{GREEN}━━━ DX v3.0 Warm Install (With Cache) ━━━{NC}")
    dx_warm_times = []
    for i in range(3):
        clean_cache(full=False)  # Keep cache, only remove node_modules
        print(f"Installing with DX v3 (run {i+1}/3)... ", end='', flush=True)
        elapsed, _ = run_command(f'"{DX_BIN}" install --v3', cwd=TEST_DIR)
        dx_warm_times.append(elapsed)
        print(f"✓ {elapsed:.3f}s")
    
    dx_warm_avg = sum(dx_warm_times) / len(dx_warm_times)
    print()
    print(f"{GREEN}DX Warm Average: {dx_warm_avg:.3f}s{NC}")
    print()
    
    # Calculate warm speedup
    warm_speedup = bun_warm_avg / dx_warm_avg if dx_warm_avg > 0 else 0
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print(f"{CYAN}Warm Start Result: {warm_speedup:.2f}x faster than Bun{NC}")
    if warm_speedup >= 50.0:
        print(f"{GREEN}✅ WARM START TARGET ACHIEVED! (≥50x){NC}")
    else:
        print(f"{YELLOW}⚠️  Warm start: {warm_speedup:.2f}x (target: 50x){NC}")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print()
    
    # ═══════════════════════════════════════════════════════════════
    # FINAL SUMMARY
    # ═══════════════════════════════════════════════════════════════
    
    print()
    print("╔════════════════════════════════════════════════════════════╗")
    print("║              PRODUCTION BENCHMARK RESULTS                  ║")
    print("╠════════════════════════════════════════════════════════════╣")
    print("║                                                            ║")
    print("║  COLD START (3x target):                                  ║")
    print(f"║    Bun:        {bun_cold_avg:8.3f}s                                   ║")
    print(f"║    DX v3:      {dx_cold_avg:8.3f}s                                   ║")
    print(f"║    Speedup:    {cold_speedup:8.2f}x                                   ║")
    if cold_speedup >= 3.0:
        print("║    Status:     ✅ ACHIEVED                              ║")
    else:
        print("║    Status:     ⚠️  NOT YET                              ║")
    print("║                                                            ║")
    print("║  WARM START (50x target):                                 ║")
    print(f"║    Bun:        {bun_warm_avg:8.3f}s                                   ║")
    print(f"║    DX v3:      {dx_warm_avg:8.3f}s                                   ║")
    print(f"║    Speedup:    {warm_speedup:8.2f}x                                   ║")
    if warm_speedup >= 50.0:
        print("║    Status:     ✅ ACHIEVED                              ║")
    else:
        print("║    Status:     ⚠️  NOT YET                              ║")
    print("║                                                            ║")
    print("╠════════════════════════════════════════════════════════════╣")
    if cold_speedup >= 3.0 and warm_speedup >= 50.0:
        print("║                                                            ║")
        print("║  🎉 ALL TARGETS ACHIEVED! PRODUCTION READY! 🚀            ║")
        print("║                                                            ║")
    else:
        print("║                                                            ║")
        print("║  📊 Benchmarks complete. Review results above.            ║")
        print("║                                                            ║")
    print("╚════════════════════════════════════════════════════════════╝")
    print()
    
    # Save results
    results_file = SCRIPT_DIR / "PRODUCTION_BENCHMARK_RESULTS.md"
    with open(results_file, 'w') as f:
        f.write(f"# DX Package Manager v3.0 - Production Benchmark Results\n\n")
        f.write(f"**Date:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n")
        f.write(f"**Test Project:** Next.js dependencies (5 packages)\n\n")
        f.write(f"## Cold Start Results\n\n")
        f.write(f"| Metric | Bun | DX v3 | Speedup |\n")
        f.write(f"|--------|-----|-------|---------||\n")
        f.write(f"| Run 1 | {bun_times[0]:.3f}s | {dx_cold_times[0]:.3f}s | - |\n")
        f.write(f"| Run 2 | {bun_times[1]:.3f}s | {dx_cold_times[1]:.3f}s | - |\n")
        f.write(f"| Run 3 | {bun_times[2]:.3f}s | {dx_cold_times[2]:.3f}s | - |\n")
        f.write(f"| **Average** | **{bun_cold_avg:.3f}s** | **{dx_cold_avg:.3f}s** | **{cold_speedup:.2f}x** |\n\n")
        f.write(f"**Target:** 3x faster\n")
        f.write(f"**Status:** {'✅ ACHIEVED' if cold_speedup >= 3.0 else f'⚠️ {cold_speedup:.2f}x'}\n\n")
        f.write(f"## Warm Start Results\n\n")
        f.write(f"| Metric | Bun | DX v3 | Speedup |\n")
        f.write(f"|--------|-----|-------|---------||\n")
        f.write(f"| Run 1 | {bun_warm_times[0]:.3f}s | {dx_warm_times[0]:.3f}s | - |\n")
        f.write(f"| Run 2 | {bun_warm_times[1]:.3f}s | {dx_warm_times[1]:.3f}s | - |\n")
        f.write(f"| Run 3 | {bun_warm_times[2]:.3f}s | {dx_warm_times[2]:.3f}s | - |\n")
        f.write(f"| **Average** | **{bun_warm_avg:.3f}s** | **{dx_warm_avg:.3f}s** | **{warm_speedup:.2f}x** |\n\n")
        f.write(f"**Target:** 50x faster\n")
        f.write(f"**Status:** {'✅ ACHIEVED' if warm_speedup >= 50.0 else f'⚠️ {warm_speedup:.2f}x'}\n\n")
        f.write(f"## Overall Assessment\n\n")
        if cold_speedup >= 3.0 and warm_speedup >= 50.0:
            f.write(f"**🎉 ALL TARGETS ACHIEVED! PRODUCTION READY! 🚀**\n\n")
        else:
            f.write(f"**Status:** Benchmarks complete. See results above for details.\n\n")
    
    print(f"{CYAN}Results saved to: {results_file}{NC}")
    print()
    
    return 0 if (cold_speedup >= 3.0 and warm_speedup >= 50.0) else 1

if __name__ == "__main__":
    sys.exit(main())
