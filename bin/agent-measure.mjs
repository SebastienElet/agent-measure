#!/usr/bin/env bun
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const variants = { "darwin-arm64": "darwin-arm64", "linux-x64": "linux-x64-gnu" };
const version = "0.0.1";
const suffix = variants[`${process.platform}-${process.arch}`];
try {
  if (!suffix) throw new Error(`Unsupported platform: ${process.platform}-${process.arch}`);
  if (process.platform === "linux" && !process.report.getReport().header.glibcVersionRuntime) {
    throw new Error("Linux requires glibc");
  }
  let executable;
  try {
    const manifest = fileURLToPath(import.meta.resolve(`@sebastienelet/agent-measure-${suffix}/package.json`));
    executable = join(dirname(manifest), "bin/agent-measure");
    if ((await Bun.file(manifest).json()).version !== version || !(await Bun.file(executable).exists())) {
      throw new Error("Invalid native variant");
    }
  } catch {
    throw new Error(`Missing @sebastienelet/agent-measure-${suffix}@${version}; install optional dependencies with Bun`);
  }
  const result = Bun.spawnSync([executable, ...process.argv.slice(2)], { stdin: "inherit", stdout: "inherit", stderr: "inherit" });
  if (result.error) throw result.error;
  if (result.signalCode) process.kill(process.pid, result.signalCode);
  process.exit(result.exitCode ?? 1);
} catch (error) {
  console.error(`agent-measure: ${error.message}`);
  process.exit(1);
}
