#!/usr/bin/env bun
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const variants = { "darwin-arm64": "darwin-arm64", "linux-x64": "linux-x64-gnu" };
const suffix = variants[`${process.platform}-${process.arch}`];
try {
  if (!suffix) throw new Error(`Unsupported platform: ${process.platform}-${process.arch}`);
  if (process.platform === "linux" && !process.report.getReport().header.glibcVersionRuntime) {
    throw new Error("Linux requires glibc");
  }
  let manifest;
  try {
    manifest = import.meta.resolve(`@sebastienelet/agent-measure-${suffix}/package.json`);
  } catch {
    throw new Error(`Missing @sebastienelet/agent-measure-${suffix}@0.0.1; install optional dependencies with Bun`);
  }
  const executable = join(dirname(fileURLToPath(manifest)), "bin/agent-measure");
  const result = Bun.spawnSync([executable, ...process.argv.slice(2)], { stdin: "inherit", stdout: "inherit", stderr: "inherit" });
  if (result.error) throw result.error;
  if (result.signalCode) process.kill(process.pid, result.signalCode);
  process.exit(result.exitCode ?? 1);
} catch (error) {
  console.error(`agent-measure: ${error.message}`);
  process.exit(1);
}
